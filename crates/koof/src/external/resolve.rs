use super::http::Auth;
use crate::{
    error::{CredentialError, ResolveValueError},
    reference::{ResourceReference, SecretReference},
    resource::{
        AuthScheme, CredentialSource, ManagedResource, NamespacePolicy, ResourceDescriptor,
        ValueExpr,
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

    fn require_namespace(&self, purpose: &str) -> Result<String, ResolveValueError> {
        self.namespace.clone().ok_or_else(|| {
            ResolveValueError::Missing(format!(
                "{} is cluster-scoped; {purpose} needs an explicit namespace",
                self.descriptor.kind
            ))
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
                    ResolveValueError::Missing(format!(
                        "{} has no identifier {name}",
                        target.descriptor.kind
                    ))
                })?;
                resolve_value(client, &target, identifier)
                    .await?
                    .map_or_else(
                        || {
                            Err(ResolveValueError::ReferenceNotReady(format!(
                                "{} referenced via {} has no {name} yet",
                                target.descriptor.kind,
                                via.join(".")
                            )))
                        },
                        |value| Ok(Some(value)),
                    )
            }
        }
    })
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
            ResolveValueError::Missing(format!(
                "{} has no reference {relation}",
                current.descriptor.kind
            ))
        })?;
        let target: ResourceReference = non_null(&current.value, reference.from)
            .cloned()
            .map(serde_json::from_value)
            .transpose()?
            .ok_or_else(|| {
                ResolveValueError::Missing(format!(
                    "resource reference {} is not set",
                    reference.from
                ))
            })?;

        let api_resource = (reference.target.api_resource)();
        let (api, namespace): (Api<DynamicObject>, Option<String>) = if !reference.target.namespaced
            || reference.namespace == NamespacePolicy::ClusterScoped
        {
            (Api::all_with(client.clone(), &api_resource), None)
        } else {
            let namespace = match reference.namespace {
                NamespacePolicy::FromReference => match target.namespace.clone() {
                    Some(namespace) => namespace,
                    None => current.require_namespace("the reference")?,
                },
                _ => current.require_namespace("the reference")?,
            };
            (
                Api::namespaced_with(client.clone(), &namespace, &api_resource),
                Some(namespace),
            )
        };

        let object = api.get(&target.name).await?;
        current = Node {
            descriptor: reference.target,
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
    let credentials = node.descriptor.credentials.as_ref().ok_or_else(|| {
        CredentialError::MissingValue(format!("{} has no credentials", node.descriptor.kind))
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
        return Err(CredentialError::MissingValue(format!(
            "credentials of {} do not end at a Secret",
            node.descriptor.kind
        )));
    };

    let selector: SecretReference = non_null(&terminal.value, path)
        .cloned()
        .map(serde_json::from_value)
        .transpose()
        .map_err(ResolveValueError::from)?
        .ok_or_else(|| {
            CredentialError::MissingValue(format!("credential field {path} is not set"))
        })?;
    let namespace = match selector.namespace.clone() {
        Some(namespace) => namespace,
        None => terminal.require_namespace("the credentials Secret")?,
    };
    let value = resolve_secret_key(client, &namespace, &selector.name, &selector.key).await?;

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
        .ok_or_else(|| {
            CredentialError::MissingValue(format!(
                "Secret {namespace}/{name} does not contain key {key:?}"
            ))
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
