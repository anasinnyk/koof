use super::http::Auth;
use crate::{
    error::{CredentialError, ResolveValueError},
    reference::{ClusterSecretKeySelector, SecretKeySelector},
    resource::{
        AuthScheme, CredentialSource, ManagedResource, Reference, ResourceDescriptor, ValueExpr,
    },
};
use futures::future::BoxFuture;
use k8s_openapi::api::core::v1::Secret;
use kube::api::{Api, DynamicObject};
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Node {
    pub descriptor: &'static ResourceDescriptor,
    pub namespace: Option<String>,
    pub value: Value,
}

impl Node {
    /// # Errors
    ///
    /// Returns [`ResolveValueError`] when the resource cannot be serialized.
    pub fn of<R: ManagedResource>(resource: &R) -> Result<Self, ResolveValueError> {
        Ok(Self {
            descriptor: R::descriptor(),
            namespace: kube::ResourceExt::namespace(resource),
            value: serde_json::to_value(resource)?,
        })
    }
}

/// Resolves `expression` against `node`; `Ok(None)` when the value is not set.
///
/// # Errors
///
/// Returns [`ResolveValueError`] when a field is not a scalar, a reference is not set
/// or points nowhere, or an identifier is unknown.
#[must_use = "the future does nothing unless awaited"]
pub fn resolve_value<'a>(
    client: &'a kube::Client,
    node: &'a Node,
    expression: &'a ValueExpr,
) -> BoxFuture<'a, Result<Option<String>, ResolveValueError>> {
    Box::pin(async move {
        match expression {
            ValueExpr::Field(path) => scalar(&node.value, path),
            ValueExpr::ExternalName => Ok(node
                .descriptor
                .external_name(&node.value)
                .map(str::to_owned)),
            ValueExpr::Literal(value) => Ok(Some((*value).to_owned())),
            ValueExpr::Coalesce(values) => {
                for value in *values {
                    if let Some(resolved) = resolve_value(client, node, value).await? {
                        return Ok(Some(resolved));
                    }
                }
                Ok(None)
            }
            ValueExpr::RelatedIdentifier { via, name } => {
                let target = traverse(client, node, via).await?;
                let identifier = target.descriptor.identifier(name).ok_or_else(|| {
                    ResolveValueError::UnknownIdentifier {
                        kind: target.descriptor.kind,
                        name,
                    }
                })?;
                resolve_value(client, &target, identifier)
                    .await?
                    .map_or_else(
                        || {
                            Err(ResolveValueError::ReferenceNotReady {
                                kind: target.descriptor.kind,
                                via: via.join("."),
                                identifier: name,
                            })
                        },
                        |value| Ok(Some(value)),
                    )
            }
        }
    })
}

#[derive(serde::Deserialize)]
struct KindedReference {
    name: String,
    kind: Option<String>,
}

/// The kind and name `reference` points at in `value`; `Ok(None)` when it is not set.
///
/// # Errors
///
/// Returns [`ResolveValueError`] when the reference is malformed or names a kind it
/// cannot point at.
pub fn target(
    value: &Value,
    reference: &Reference,
) -> Result<Option<(&'static ResourceDescriptor, String)>, ResolveValueError> {
    let Some(target) = non_null(value, reference.from)
        .cloned()
        .map(serde_json::from_value::<KindedReference>)
        .transpose()?
    else {
        return Ok(None);
    };
    let descriptor = match target.kind.as_deref() {
        None => reference.targets.first(),
        Some(kind) => reference
            .targets
            .iter()
            .find(|descriptor| descriptor.kind == kind),
    }
    .copied()
    .ok_or_else(|| ResolveValueError::ReferenceKind {
        from: reference.from,
        kind: target.kind.clone().unwrap_or_default(),
    })?;
    Ok(Some((descriptor, target.name)))
}

/// Follows the named references from `node` and returns the resource at the end.
///
/// # Errors
///
/// Returns [`ResolveValueError`] when a reference is unknown, not set, malformed, or
/// the referenced object cannot be read.
pub async fn traverse(
    client: &kube::Client,
    node: &Node,
    via: &[&str],
) -> Result<Node, ResolveValueError> {
    let mut current = node.clone();
    for relation in via {
        let reference = current.descriptor.reference(relation).ok_or_else(|| {
            ResolveValueError::UnknownReference {
                kind: current.descriptor.kind,
                name: (*relation).to_owned(),
            }
        })?;
        let (descriptor, name) = target(&current.value, reference)?.ok_or_else(|| {
            ResolveValueError::ReferenceNotSet {
                from: reference.from,
            }
        })?;

        let api_resource = (descriptor.api_resource)();
        let (api, namespace): (Api<DynamicObject>, Option<String>) =
            match (descriptor.namespaced, current.namespace.clone()) {
                (false, _) => (Api::all_with(client.clone(), &api_resource), None),
                (true, Some(namespace)) => (
                    Api::namespaced_with(client.clone(), &namespace, &api_resource),
                    Some(namespace),
                ),
                (true, None) => {
                    return Err(ResolveValueError::ClusterScopedReference {
                        kind: current.descriptor.kind,
                        target: descriptor.kind,
                    });
                }
            };

        let object = api.get(&name).await?;
        current = Node {
            descriptor,
            namespace,
            value: serde_json::to_value(object)?,
        };
    }
    Ok(current)
}

/// Credentials for `node`: its own Secret, or the Secret of the resource its
/// `Related` chain ends at, applied with `node`'s auth scheme.
///
/// # Errors
///
/// Returns [`CredentialError`] when the resource has no credentials, the chain does
/// not end at Secret credentials, the selector is not set, or the Secret cannot be
/// read.
pub async fn resolve_credentials(
    client: &kube::Client,
    node: &Node,
) -> Result<Auth, CredentialError> {
    let credentials =
        node.descriptor
            .credentials
            .as_ref()
            .ok_or_else(|| CredentialError::NoCredentials {
                kind: node.descriptor.kind,
            })?;

    let terminal = match &credentials.source {
        CredentialSource::Secret { .. } => node.clone(),
        CredentialSource::Related { via } => traverse(client, node, via).await?,
    };
    let Some(CredentialSource::Secret { path }) = terminal
        .descriptor
        .credentials
        .as_ref()
        .map(|credentials| &credentials.source)
    else {
        return Err(CredentialError::NotSecret {
            kind: node.descriptor.kind,
        });
    };

    let selector = non_null(&terminal.value, path)
        .cloned()
        .ok_or_else(|| CredentialError::FieldNotSet { path })?;
    let secret = match terminal.namespace {
        Some(namespace) => {
            let SecretKeySelector { name, key } =
                serde_json::from_value(selector).map_err(ResolveValueError::from)?;
            ClusterSecretKeySelector {
                name,
                key,
                namespace,
            }
        }
        None => serde_json::from_value(selector).map_err(ResolveValueError::from)?,
    };
    let value = resolve_secret_key(client, &secret.namespace, &secret.name, &secret.key).await?;

    Ok(match credentials.auth {
        AuthScheme::Bearer => Auth::Bearer(value),
        AuthScheme::Header(name) => Auth::Header {
            name: name.to_owned(),
            value,
        },
    })
}

/// Reads `key` of Secret `name` in `namespace` as UTF-8.
///
/// # Errors
///
/// Returns [`CredentialError`] when the Secret cannot be read, has no such key, or
/// the value is not UTF-8.
pub async fn resolve_secret_key(
    client: &kube::Client,
    namespace: &str,
    name: &str,
    key: &str,
) -> Result<String, CredentialError> {
    let secrets: Api<Secret> = Api::namespaced(client.clone(), namespace);
    let secret = secrets.get(name).await?;

    if let Some(value) = secret.string_data.as_ref().and_then(|data| data.get(key)) {
        return Ok(value.clone());
    }

    let bytes = secret
        .data
        .as_ref()
        .and_then(|data| data.get(key))
        .ok_or_else(|| CredentialError::SecretKeyMissing {
            secret: format!("{namespace}/{name}"),
            key: key.to_owned(),
        })?
        .0
        .clone();

    String::from_utf8(bytes).map_err(|source| CredentialError::InvalidSecretUtf8 {
        secret: format!("{namespace}/{name}"),
        key: key.to_owned(),
        source,
    })
}

pub(crate) fn non_null<'a>(value: &'a Value, pointer: &str) -> Option<&'a Value> {
    value.pointer(pointer).filter(|value| !value.is_null())
}

fn scalar(value: &Value, pointer: &str) -> Result<Option<String>, ResolveValueError> {
    match value.pointer(pointer) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(Value::Number(value)) => Ok(Some(value.to_string())),
        Some(Value::Bool(value)) => Ok(Some(value.to_string())),
        Some(_) => Err(ResolveValueError::NonScalar {
            path: pointer.to_owned(),
        }),
    }
}
