use crate::{error::ReconcileError, external::resolve::non_null, resource::ManagedResource};
use k8s_openapi::{ByteString, api::core::v1::Secret};
use kube::{
    Api, ResourceExt as _,
    api::{ObjectMeta, Patch, PatchParams, PostParams},
};
use serde_json::Value;
use std::collections::BTreeMap;

pub const SECRET_REF: &str = "/spec/writeConnectionSecretToRef";

/// The connection details of the descriptor that `external` (the resource as the API
/// returns it) holds, by Secret key.
#[must_use]
pub fn details(
    connection_secret: &[(&'static str, &'static str)],
    external: &Value,
) -> BTreeMap<String, String> {
    connection_secret
        .iter()
        .filter_map(|(key, pointer)| {
            let value = match non_null(external, pointer)? {
                Value::String(value) => value.clone(),
                Value::Number(value) => value.to_string(),
                Value::Bool(value) => value.to_string(),
                _ => return None,
            };
            Some(((*key).to_owned(), value))
        })
        .collect()
}

/// Writes `details` to the Secret named by `spec.writeConnectionSecretToRef`,
/// creating it owned by `resource`; keys not in `details` are kept, so values the
/// API returns only on create survive later observations. Returns whether the
/// Secret changed.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the reference is malformed, the Secret belongs
/// to another object, or it cannot be read or written.
pub async fn write<R: ManagedResource>(
    client: &kube::Client,
    field_manager: &str,
    resource: &R,
    details: &BTreeMap<String, String>,
) -> Result<bool, ReconcileError> {
    let Some((namespace, name)) = target(resource)? else {
        return Ok(false);
    };
    if details.is_empty() {
        return Ok(false);
    }
    let secret = format!("{namespace}/{name}");
    let failed = |source| ReconcileError::ConnectionSecret {
        secret: secret.clone(),
        source: Box::new(source),
    };
    let data: BTreeMap<String, ByteString> = details
        .iter()
        .map(|(key, value)| (key.clone(), ByteString(value.clone().into_bytes())))
        .collect();
    let api: Api<Secret> = Api::namespaced(client.clone(), &namespace);
    let Some(existing) = api.get_opt(&name).await.map_err(failed)? else {
        let created = Secret {
            metadata: ObjectMeta {
                name: Some(name.clone()),
                namespace: Some(namespace.clone()),
                owner_references: resource.controller_owner_ref(&()).map(|owner| vec![owner]),
                ..ObjectMeta::default()
            },
            data: Some(data),
            ..Secret::default()
        };
        let parameters = PostParams {
            field_manager: Some(field_manager.to_owned()),
            ..PostParams::default()
        };
        api.create(&parameters, &created).await.map_err(failed)?;
        return Ok(true);
    };
    if !owned_by(&existing, resource) {
        return Err(ReconcileError::ConnectionSecretNotOwned { secret });
    }
    let current = existing.data.unwrap_or_default();
    let changed: BTreeMap<&String, &ByteString> = data
        .iter()
        .filter(|(key, value)| current.get(*key) != Some(*value))
        .collect();
    if changed.is_empty() {
        return Ok(false);
    }
    let patch = serde_json::json!({ "data": changed });
    api.patch(&name, &PatchParams::default(), &Patch::Merge(patch))
        .await
        .map_err(failed)?;
    Ok(true)
}

/// Whether `resource` asks for a connection Secret (`spec.writeConnectionSecretToRef`).
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource cannot be serialized or the
/// reference is malformed.
pub fn requested<R: ManagedResource>(resource: &R) -> Result<bool, ReconcileError> {
    Ok(target(resource)?.is_some())
}

fn target<R: ManagedResource>(resource: &R) -> Result<Option<(String, String)>, ReconcileError> {
    #[derive(serde::Deserialize)]
    struct Reference {
        name: String,
        namespace: Option<String>,
    }
    let value = serde_json::to_value(resource)?;
    let Some(reference) = non_null(&value, SECRET_REF) else {
        return Ok(None);
    };
    let reference: Reference = serde_json::from_value(reference.clone())?;
    let namespace = resource
        .namespace()
        .or(reference.namespace)
        .unwrap_or_default();
    Ok(Some((namespace, reference.name)))
}

fn owned_by<R: ManagedResource>(secret: &Secret, resource: &R) -> bool {
    resource.meta().uid.as_ref().is_none_or(|uid| {
        secret
            .metadata
            .owner_references
            .iter()
            .flatten()
            .any(|owner| &owner.uid == uid)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::{Cluster, NAMESPACE, record};
    use pretty_assertions::assert_eq;
    use serde_json::json;

    const KEYS: &[(&str, &str)] = &[("id", "/id"), ("ttl", "/ttl"), ("dsn", "/dsn/public")];

    fn written(cluster: &Cluster) -> Value {
        cluster.get("Secret", "conn").map_or(Value::Null, |secret| {
            json!({ "data": secret["data"], "owners": secret["metadata"]["ownerReferences"] })
        })
    }

    fn details_of(external: &Value) -> BTreeMap<String, String> {
        details(KEYS, external)
    }

    fn record_writing_to(secret: &str) -> crate::testing::Record {
        let mut record = record(Some("record-1"), json!({ "name": "www" }));
        record.metadata.uid = Some("uid-1".to_owned());
        record.spec.write_connection_secret_to_ref = Some(crate::reference::SecretReference {
            name: secret.to_owned(),
        });
        record
    }

    fn secret(owner: Option<&str>, data: &Value) -> Value {
        json!({
            "apiVersion": "v1",
            "kind": "Secret",
            "metadata": {
                "name": "conn",
                "namespace": NAMESPACE,
                "ownerReferences": owner.map(|uid| json!([{
                    "apiVersion": "example.dev/v1", "kind": "Record", "name": "www", "uid": uid,
                }])),
            },
            "data": data,
        })
    }

    #[test]
    fn details_are_the_scalars_found_at_their_pointers() {
        assert_eq!(
            details_of(&json!({ "id": "r1", "ttl": 300, "dsn": { "public": null } })),
            BTreeMap::from([
                ("id".to_owned(), "r1".to_owned()),
                ("ttl".to_owned(), "300".to_owned()),
            ])
        );
    }

    #[tokio::test]
    async fn creates_the_secret_owned_by_the_resource() -> Result<(), ReconcileError> {
        let cluster = Cluster::new(&[]);
        let details = details_of(&json!({ "id": "r1" }));

        let changed = write(
            &cluster.client(),
            "koof",
            &record_writing_to("conn"),
            &details,
        )
        .await?;

        assert_eq!(
            (changed, written(&cluster)),
            (
                true,
                json!({
                    "data": { "id": "cjE=" },
                    "owners": [{
                        "apiVersion": "example.dev/v1",
                        "kind": "Record",
                        "name": "www",
                        "uid": "uid-1",
                        "controller": true,
                    }],
                })
            )
        );
        Ok(())
    }

    #[tokio::test]
    async fn keeps_keys_it_does_not_observe_and_updates_the_others() -> Result<(), ReconcileError> {
        let cluster = Cluster::new(&[secret(
            Some("uid-1"),
            &json!({ "id": "b2xk", "secret": "czM=" }),
        )]);
        let details = details_of(&json!({ "id": "r1" }));

        let changed = write(
            &cluster.client(),
            "koof",
            &record_writing_to("conn"),
            &details,
        )
        .await?;

        assert_eq!(
            (changed, written(&cluster)["data"].clone()),
            (true, json!({ "id": "cjE=", "secret": "czM=" }))
        );
        Ok(())
    }

    #[tokio::test]
    async fn leaves_an_up_to_date_secret_alone() -> Result<(), ReconcileError> {
        let cluster = Cluster::new(&[secret(Some("uid-1"), &json!({ "id": "cjE=" }))]);
        let details = details_of(&json!({ "id": "r1" }));

        let changed = write(
            &cluster.client(),
            "koof",
            &record_writing_to("conn"),
            &details,
        )
        .await?;

        assert!(!changed);
        Ok(())
    }

    #[tokio::test]
    async fn refuses_a_secret_of_another_object() {
        let cluster = Cluster::new(&[secret(None, &json!({ "id": "b2xk" }))]);
        let details = details_of(&json!({ "id": "r1" }));

        let result = write(
            &cluster.client(),
            "koof",
            &record_writing_to("conn"),
            &details,
        )
        .await;

        assert_eq!(
            result.map_err(|error| error.to_string()),
            Err(format!(
                "Secret {NAMESPACE}/conn is not owned by this resource; delete it or set another \
                 writeConnectionSecretToRef"
            ))
        );
    }

    #[tokio::test]
    async fn writes_nothing_without_a_reference() -> Result<(), ReconcileError> {
        let cluster = Cluster::new(&[]);
        let record = record(Some("record-1"), json!({ "name": "www" }));

        let changed = write(
            &cluster.client(),
            "koof",
            &record,
            &details_of(&json!({ "id": "r1" })),
        )
        .await?;

        assert_eq!((changed, written(&cluster)), (false, Value::Null));
        Ok(())
    }
}
