use crate::error::ResolveValueError;
use serde_json::{Map, Value};

fn segments(path: &str) -> impl Iterator<Item = &str> {
    path.split('.').filter(|segment| !segment.is_empty())
}

#[must_use]
pub fn get<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    segments(path).try_fold(value, |current, segment| current.get(segment))
}

#[must_use]
pub fn get_non_null<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    get(value, path).filter(|value| !value.is_null())
}

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
