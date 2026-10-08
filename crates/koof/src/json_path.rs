//! Dot paths such as `spec.forProvider.zoneRef` into JSON values. Empty segments
//! are ignored, so the empty path addresses the whole value.

use crate::error::ResolveValueError;
use serde_json::{Map, Value};

fn segments(path: &str) -> impl Iterator<Item = &str> {
    path.split('.').filter(|segment| !segment.is_empty())
}

/// Value at `path`.
#[must_use]
pub fn get<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    segments(path).try_fold(value, |current, segment| current.get(segment))
}

/// Value at `path`; `None` when missing or `null`.
#[must_use]
pub fn get_non_null<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    get(value, path).filter(|value| !value.is_null())
}

/// Mutable value at `path`.
#[must_use]
pub fn get_mut<'a>(value: &'a mut Value, path: &str) -> Option<&'a mut Value> {
    segments(path).try_fold(value, |current, segment| current.get_mut(segment))
}

/// Scalar at `path` as a string; `Ok(None)` when missing or `null`.
///
/// # Errors
///
/// Returns [`ResolveValueError::NonScalar`] when the value is an object or array.
pub fn scalar(value: &Value, path: &str) -> Result<Option<String>, ResolveValueError> {
    match get(value, path) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        Some(Value::Number(value)) => Ok(Some(value.to_string())),
        Some(Value::Bool(value)) => Ok(Some(value.to_string())),
        Some(_) => Err(ResolveValueError::NonScalar {
            path: path.to_owned(),
        }),
    }
}

/// Writes `value` at `path`, creating intermediate objects (and replacing
/// non-objects on the way).
pub fn set(target: &mut Value, path: &str, value: Value) {
    let mut current = target;
    let mut segments = segments(path).peekable();
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn get_follows_dot_paths() {
        let value = json!({ "spec": { "forProvider": { "name": "a" } } });
        assert_eq!(get(&value, "spec.forProvider.name"), Some(&json!("a")));
        assert_eq!(get(&value, ""), Some(&value));
        assert_eq!(get(&value, "spec.missing"), None);
    }

    #[test]
    fn set_creates_intermediate_objects() {
        let mut value = Value::Null;
        set(&mut value, "metadata.resourceVersion", json!("1"));
        assert_eq!(value, json!({ "metadata": { "resourceVersion": "1" } }));
    }

    #[test]
    fn scalar_rejects_objects() {
        let value = json!({ "a": { "b": 1 }, "n": 2, "z": null });
        assert_eq!(scalar(&value, "a.b").ok().flatten().as_deref(), Some("1"));
        assert_eq!(scalar(&value, "z").ok().flatten(), None);
        assert!(scalar(&value, "a").is_err());
    }
}
