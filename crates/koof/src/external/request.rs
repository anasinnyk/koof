use super::resolve::{Node, resolve_value};
use crate::{
    error::{ReconcileError, ResolveValueError},
    json_path,
    resource::RequestMapping,
};
use serde_json::{Map, Value};

/// Request body: the object at `mapping.from` with `mapping.set` applied,
/// deserialized into `T` (unknown fields are dropped).
///
/// # Errors
///
/// Returns [`ReconcileError`] when `from` is not an object, a `set` value cannot be
/// resolved, or the JSON does not match `T`.
pub async fn build<T: serde::de::DeserializeOwned>(
    client: &kube::Client,
    node: &Node,
    mapping: &RequestMapping,
) -> Result<T, ReconcileError> {
    let mut body = match json_path::get(&node.value, mapping.from) {
        Some(Value::Object(object)) => Value::Object(object.clone()),
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
            .ok_or_else(|| {
                ResolveValueError::Missing(format!("request field {path} could not be resolved"))
            })?;
        json_path::set(&mut body, path, Value::String(value));
    }
    serde_json::from_value(body).map_err(|error| {
        ReconcileError::InvalidRequest(format!(
            "request body built from {} does not match the API: {error}",
            mapping.from
        ))
    })
}
