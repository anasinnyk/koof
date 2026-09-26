//! Building provider request bodies from a managed resource.

use crate::{
    error::{ReconcileError, ResolveValueError},
    reference::value_at,
    resolve::{Node, resolve_value},
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
    let mut body = match value_at(&node.value, mapping.from) {
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
        set_path(&mut body, path, Value::String(value));
    }
    serde_json::from_value(body).map_err(|error| {
        ReconcileError::InvalidRequest(format!(
            "request body built from {} does not match the API: {error}",
            mapping.from
        ))
    })
}

/// Writes `value` at a dot path, creating intermediate objects.
fn set_path(target: &mut Value, path: &str, value: Value) {
    let mut current = target;
    let mut segments = path.split('.').peekable();
    while let Some(segment) = segments.next() {
        if !current.is_object() {
            *current = Value::Object(Map::new());
        }
        let Value::Object(object) = current else {
            return;
        };
        if segments.peek().is_none() {
            object.insert(segment.to_owned(), value);
            return;
        }
        current = object
            .entry(segment.to_owned())
            .or_insert_with(|| Value::Object(Map::new()));
    }
}
