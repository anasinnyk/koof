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
#[serde(rename_all = "camelCase")]
struct KindedReference {
    name: String,
    kind: Option<String>,
    api_version: Option<String>,
}

/// The kind and name `reference` points at in `value`; `Ok(None)` when it is not set.
/// `kind` defaults to the kind of the first target; `apiVersion` tells apart targets
/// of the same kind in different API groups.
///
/// # Errors
///
/// Returns [`ResolveValueError`] when the reference is malformed, names a kind it
/// cannot point at, or matches several targets.
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
    let kind = target
        .kind
        .as_deref()
        .or_else(|| reference.targets.first().map(|descriptor| descriptor.kind))
        .unwrap_or_default();
    let api_version = target.api_version.as_deref();
    let mut matching = reference.targets.iter().copied().filter(|descriptor| {
        descriptor.kind == kind
            && api_version.is_none_or(|version| (descriptor.api_resource)().api_version == version)
    });
    let qualified = || match api_version {
        Some(version) => format!("{version}, kind={kind}"),
        None => kind.to_owned(),
    };
    let descriptor = matching
        .next()
        .ok_or_else(|| ResolveValueError::ReferenceKind {
            from: reference.from,
            kind: qualified(),
        })?;
    if matching.next().is_some() {
        return Err(ResolveValueError::AmbiguousReference {
            from: reference.from,
            kind: qualified(),
        });
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::{ObserveDescriptor, OperationDescriptor};
    use serde_json::json;

    fn api_resource(group: &str) -> kube::api::ApiResource {
        kube::api::ApiResource {
            group: group.to_owned(),
            version: "v1".to_owned(),
            api_version: format!("{group}/v1"),
            kind: "Record".to_owned(),
            plural: "records".to_owned(),
        }
    }

    static ZONES: ResourceDescriptor = ResourceDescriptor::new(
        "Record",
        || api_resource("zones.example.dev"),
        "zones.example.dev",
        true,
        ObserveDescriptor::new(OperationDescriptor::new()),
    );
    static DNS: ResourceDescriptor = ResourceDescriptor::new(
        "Record",
        || api_resource("dns.example.dev"),
        "dns.example.dev",
        true,
        ObserveDescriptor::new(OperationDescriptor::new()),
    );
    static REFERENCE: Reference = Reference::new("record", "/ref", &[&ZONES, &DNS]);

    fn resolve(reference: &Value) -> Result<Option<&'static str>, ResolveValueError> {
        Ok(target(&json!({ "ref": reference }), &REFERENCE)?
            .map(|(descriptor, _)| descriptor.group))
    }

    #[test]
    fn same_kind_in_two_groups_needs_api_version() -> Result<(), ResolveValueError> {
        assert!(matches!(
            resolve(&json!({ "name": "a" })),
            Err(ResolveValueError::AmbiguousReference { .. })
        ));
        assert_eq!(
            resolve(&json!({ "name": "a", "apiVersion": "dns.example.dev/v1" }))?,
            Some("dns.example.dev")
        );
        assert_eq!(
            resolve(
                &json!({ "name": "a", "kind": "Record", "apiVersion": "zones.example.dev/v1" })
            )?,
            Some("zones.example.dev")
        );
        assert!(matches!(
            resolve(&json!({ "name": "a", "apiVersion": "other.example.dev/v1" })),
            Err(ResolveValueError::ReferenceKind { .. })
        ));
        Ok(())
    }

    mod values {
        use super::*;
        use crate::testing::{ZONE_ID, account, kubernetes, record, secret, zone};
        use pretty_assertions::assert_eq;
        use rstest::rstest;

        async fn resolved(
            external_name: Option<&str>,
            for_provider: Value,
            objects: &[Value],
            expression: &ValueExpr,
        ) -> Result<Option<String>, String> {
            let node = Node::of(&record(external_name, for_provider)).expect("node");
            resolve_value(&kubernetes(objects), &node, expression)
                .await
                .map_err(|error| error.to_string())
        }

        fn www() -> Value {
            json!({
                "name": "www.example.com",
                "ttl": 300,
                "proxied": true,
                "settings": { "ipv4_only": true },
                "zoneRef": { "name": "example" },
            })
        }

        #[rstest]
        #[case::string_field(ValueExpr::Field("/spec/forProvider/name"), Some("www.example.com"))]
        #[case::number_field(ValueExpr::Field("/spec/forProvider/ttl"), Some("300"))]
        #[case::boolean_field(ValueExpr::Field("/spec/forProvider/proxied"), Some("true"))]
        #[case::unset_field(ValueExpr::Field("/spec/forProvider/comment"), None)]
        #[case::external_name(ValueExpr::ExternalName, Some("record-1"))]
        #[case::literal(ValueExpr::Literal("v4"), Some("v4"))]
        #[case::first_value_that_is_set(
            ValueExpr::Coalesce(&[ValueExpr::Field("/spec/forProvider/comment"), ValueExpr::ExternalName]),
            Some("record-1")
        )]
        #[case::id_of_the_referenced_resource(ZONE_ID, Some("zone-1"))]
        #[tokio::test]
        async fn resolves_values_of_the_resource(
            #[case] expression: ValueExpr,
            #[case] value: Option<&str>,
        ) {
            assert_eq!(
                resolved(Some("record-1"), www(), &[zone()], &expression).await,
                Ok(value.map(str::to_owned))
            );
        }

        #[tokio::test]
        async fn falls_back_to_the_object_name_before_the_first_create() {
            let name_until_created =
                ValueExpr::Coalesce(&[ValueExpr::ExternalName, ValueExpr::Field("/metadata/name")]);
            assert_eq!(
                resolved(None, www(), &[], &name_until_created).await,
                Ok(Some("www".to_owned()))
            );
        }

        #[rstest]
        #[case::object_is_not_a_scalar(
            ValueExpr::Field("/spec/forProvider/settings"),
            json!({ "settings": { "ipv4_only": true } }),
            &[],
            "field \"/spec/forProvider/settings\" is not a scalar value"
        )]
        #[case::reference_not_set(ZONE_ID, json!({}), &[], "resource reference /spec/forProvider/zoneRef is not set")]
        #[case::referenced_object_not_created_yet(
            ZONE_ID,
            json!({ "zoneRef": { "name": "example" } }),
            &[object("Zone", "example", None, json!({ "forProvider": { "name": "example.com" } }))],
            "reference not ready: Zone referenced via zone has no external_id yet"
        )]
        #[tokio::test]
        async fn explains_what_is_missing(
            #[case] expression: ValueExpr,
            #[case] for_provider: Value,
            #[case] objects: &[Value],
            #[case] error: &str,
        ) {
            assert_eq!(
                resolved(Some("record-1"), for_provider, objects, &expression).await,
                Err(error.to_owned())
            );
        }

        #[tokio::test]
        async fn referenced_object_missing_from_the_cluster_is_waited_for() {
            let node = Node::of(&record(None, www())).expect("node");

            let error = resolve_value(&kubernetes(&[]), &node, &ZONE_ID)
                .await
                .expect_err("missing zone");

            assert_eq!(
                crate::retry::classify(&error.into()),
                crate::retry::Failure::Waiting
            );
        }

        fn object(kind: &str, name: &str, external_name: Option<&str>, spec: Value) -> Value {
            crate::testing::object(kind, name, external_name, spec)
        }

        async fn credentials(node: Node, objects: &[Value]) -> Result<Auth, String> {
            resolve_credentials(&kubernetes(objects), &node)
                .await
                .map_err(|error| error.to_string())
        }

        #[tokio::test]
        async fn token_comes_from_the_secret_at_the_end_of_the_chain() {
            let node = Node::of(&record(None, www())).expect("node");

            let auth = credentials(
                node,
                &[zone(), account(), secret("cloudflare", "token", "t0ken")],
            )
            .await;

            assert!(matches!(auth, Ok(Auth::Bearer(token)) if token == "t0ken"));
        }

        #[tokio::test]
        async fn each_kind_applies_the_token_with_its_own_scheme() {
            let node =
                Node::of(&serde_json::from_value::<crate::testing::Zone>(zone()).expect("zone"))
                    .expect("node");

            let auth =
                credentials(node, &[account(), secret("cloudflare", "token", "t0ken")]).await;

            assert!(
                matches!(auth, Ok(Auth::Header { name, value }) if name == "X-Auth-Key" && value == "t0ken")
            );
        }

        #[rstest]
        #[case::secret_without_the_key(
            &[zone(), account(), secret("cloudflare", "other", "t0ken")],
            "Secret default/cloudflare does not contain key \"token\""
        )]
        #[case::account_without_a_secret(
            &[zone(), object("Account", "main", Some("account-1"), json!({ "forProvider": {} }))],
            "credential field /spec/forProvider/apiTokenSecretRef is not set"
        )]
        #[tokio::test]
        async fn explains_missing_credentials(#[case] objects: &[Value], #[case] error: &str) {
            let node = Node::of(&record(None, www())).expect("node");

            assert_eq!(
                credentials(node, objects).await.err(),
                Some(error.to_owned())
            );
        }
    }
}
