use super::resolve::{Node, resolve_value};
use crate::{
    error::{ReconcileError, ResolveValueError},
    resource::RequestMapping,
};
use jsonptr::Pointer;
use serde_json::{Map, Value};

/// Request body: the object at `mapping.from` with `mapping.set` applied, or the
/// list at `mapping.from` as it is, deserialized into `T` (unknown fields are
/// dropped).
///
/// # Errors
///
/// Returns [`ReconcileError`] when `from` is neither an object nor a list without
/// `set`, a `set` value cannot be resolved, or the JSON does not match `T`.
pub async fn build<T: serde::de::DeserializeOwned>(
    client: &kube::Client,
    node: &Node,
    mapping: &RequestMapping,
) -> Result<T, ReconcileError> {
    let mut body = match node.value.pointer(mapping.from) {
        Some(Value::Object(object)) => Value::Object(without_references(node, mapping, object)),
        Some(Value::Array(items)) if mapping.set.is_empty() => Value::Array(items.clone()),
        None | Some(Value::Null) => Value::Object(Map::new()),
        Some(_) => {
            return Err(ResolveValueError::NonScalar {
                path: mapping.from.to_owned(),
            }
            .into());
        }
    };
    for (path, expression) in mapping.set {
        let value = resolve_value(client, node, expression)
            .await?
            .ok_or_else(|| ResolveValueError::RequestFieldNotSet { path })?;
        Pointer::parse(path)
            .map_err(|source| ReconcileError::RequestPath { path, source })?
            .assign(&mut body, Value::String(value))
            .map_err(|source| ReconcileError::RequestAssign { path, source })?;
    }
    serde_json::from_value(body).map_err(|source| ReconcileError::InvalidRequest {
        from: mapping.from,
        source,
    })
}

fn without_references(
    node: &Node,
    mapping: &RequestMapping,
    object: &Map<String, Value>,
) -> Map<String, Value> {
    let references: Vec<&str> = node
        .descriptor
        .references
        .iter()
        .filter_map(|reference| reference.from.strip_prefix(mapping.from)?.strip_prefix('/'))
        .collect();
    object
        .iter()
        .filter(|(field, _)| !references.contains(&field.as_str()))
        .map(|(field, value)| (field.clone(), value.clone()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        resource::ValueExpr,
        testing::{ZONE_ID, kubernetes, record, zone},
    };
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct CreateRecord {
        name: Option<String>,
        ttl: Option<u32>,
        zone: Option<ZoneId>,
    }

    #[derive(Debug, PartialEq, serde::Deserialize)]
    struct ZoneId {
        id: String,
    }

    async fn body(
        mapping: &RequestMapping,
        for_provider: serde_json::Value,
    ) -> Result<CreateRecord, String> {
        let node = Node::of(&record(Some("record-1"), for_provider)).expect("node");
        build(&kubernetes(&[zone()]), &node, mapping)
            .await
            .map_err(|error| error.to_string())
    }

    fn www() -> serde_json::Value {
        json!({ "name": "www.example.com", "ttl": 300, "zoneRef": { "name": "example" } })
    }

    #[tokio::test]
    async fn sends_for_provider_without_the_fields_the_api_does_not_know() {
        let mapping = RequestMapping::new("/spec/forProvider");

        assert_eq!(
            body(&mapping, www()).await,
            Ok(CreateRecord {
                name: Some("www.example.com".to_owned()),
                ttl: Some(300),
                zone: None
            })
        );
    }

    #[derive(Debug, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Closed {
        name: Option<String>,
        ttl: Option<u32>,
    }

    #[tokio::test]
    async fn leaves_references_out_of_the_body() {
        let node = Node::of(&record(Some("record-1"), www())).expect("node");
        let body: Result<Closed, _> = build(
            &kubernetes(&[zone()]),
            &node,
            &RequestMapping::new("/spec/forProvider"),
        )
        .await;

        assert_eq!(
            body.map_err(|error| error.to_string()),
            Ok(Closed {
                name: Some("www.example.com".to_owned()),
                ttl: Some(300)
            })
        );
    }

    #[tokio::test]
    async fn adds_the_id_of_a_referenced_resource() {
        let mapping = RequestMapping::new("/spec/forProvider").with_set(&[("/zone/id", ZONE_ID)]);

        assert_eq!(
            body(&mapping, www()).await.map(|body| body.zone),
            Ok(Some(ZoneId {
                id: "zone-1".to_owned()
            }))
        );
    }

    #[tokio::test]
    async fn fails_when_a_value_to_add_is_not_set() {
        let mapping = RequestMapping::new("/spec/forProvider")
            .with_set(&[("/zone/id", ValueExpr::Field("/spec/forProvider/zoneId"))]);

        assert_eq!(
            body(&mapping, www()).await,
            Err("request field /zone/id could not be resolved".to_owned())
        );
    }

    #[tokio::test]
    async fn fails_when_the_body_does_not_match_the_api() {
        let mapping = RequestMapping::new("/spec/forProvider")
            .with_set(&[("/ttl", ValueExpr::Literal("auto"))]);

        assert_eq!(
            body(&mapping, www()).await,
            Err("request body built from /spec/forProvider does not match the API".to_owned())
        );
    }

    #[tokio::test]
    async fn missing_source_sends_an_empty_body() {
        let mapping = RequestMapping::new("/spec/initProvider");

        assert_eq!(
            body(&mapping, www()).await,
            Ok(CreateRecord {
                name: None,
                ttl: None,
                zone: None
            })
        );
    }

    #[tokio::test]
    async fn sends_a_list_as_it_is() {
        let node = Node::of(&record(
            Some("record-1"),
            json!({ "tags": ["a", "b"], "zoneRef": { "name": "example" } }),
        ))
        .expect("node");
        let body: Result<Vec<String>, _> = build(
            &kubernetes(&[zone()]),
            &node,
            &RequestMapping::new("/spec/forProvider/tags"),
        )
        .await;

        assert_eq!(
            body.map_err(|error| error.to_string()),
            Ok(vec!["a".to_owned(), "b".to_owned()])
        );
    }
}
