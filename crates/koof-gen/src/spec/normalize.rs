use super::{resolve_ref, schema_children_mut};
use serde_json::{Map, Value};

pub fn normalize_nullable(value: &mut Value) {
    match value {
        Value::Object(map) => normalize_object(map),
        Value::Array(items) => items.iter_mut().for_each(normalize_nullable),
        _ => {}
    }
}

pub fn relax_read_only_required(document: &mut Value) {
    let schemas = document
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    relax(document, &schemas);
}

fn relax(value: &mut Value, schemas: &Map<String, Value>) {
    match value {
        Value::Object(map) => {
            for child in schema_children_mut(map) {
                relax(child, schemas);
            }
            let read_only: Vec<String> = map
                .get("properties")
                .and_then(Value::as_object)
                .into_iter()
                .flatten()
                .filter(|(_, property)| is_read_only(schemas, property))
                .map(|(name, _)| name.clone())
                .collect();
            if read_only.is_empty() {
                return;
            }
            if let Some(Value::Array(required)) = map.get_mut("required") {
                required.retain(|name| {
                    name.as_str()
                        .is_none_or(|name| !read_only.iter().any(|field| field == name))
                });
                if required.is_empty() {
                    map.remove("required");
                }
            }
        }
        Value::Array(items) => items.iter_mut().for_each(|item| relax(item, schemas)),
        _ => {}
    }
}

pub fn is_read_only(schemas: &Map<String, Value>, schema: &Value) -> bool {
    fn check(schemas: &Map<String, Value>, schema: &Value, depth: usize) -> bool {
        if depth > 16 {
            return false;
        }
        if schema.get("readOnly") == Some(&Value::Bool(true)) {
            return true;
        }
        let resolved = resolve_ref(schemas, schema);
        if !std::ptr::eq(resolved, schema) && check(schemas, resolved, depth + 1) {
            return true;
        }
        schema
            .get("allOf")
            .and_then(Value::as_array)
            .is_some_and(|parts| parts.iter().any(|part| check(schemas, part, depth + 1)))
    }
    check(schemas, schema, 0)
}

pub fn drop_non_string_enums(value: &mut Value) {
    match value {
        Value::Object(map) => {
            let drop = map
                .get("enum")
                .and_then(Value::as_array)
                .is_some_and(|values| {
                    values.iter().all(Value::is_null)
                        || values
                            .iter()
                            .any(|value| !value.is_string() && !value.is_null())
                });
            if drop && let Some(Value::Array(values)) = map.remove("enum") {
                let allowed: Vec<String> = values.iter().map(Value::to_string).collect();
                let note = format!("Allowed values: {}.", allowed.join(", "));
                let description = match map.get("description").and_then(Value::as_str) {
                    Some(existing) => format!("{existing} {note}"),
                    None => note,
                };
                map.insert("description".to_owned(), Value::String(description));
            }
            for child in schema_children_mut(map) {
                drop_non_string_enums(child);
            }
        }
        Value::Array(items) => items.iter_mut().for_each(drop_non_string_enums),
        _ => {}
    }
}

fn normalize_object(map: &mut Map<String, Value>) {
    for child in schema_children_mut(map) {
        normalize_nullable(child);
    }

    let nullable = match map.get("nullable") {
        Some(Value::Bool(nullable)) => *nullable,
        _ => return,
    };
    map.remove("nullable");
    if !nullable {
        return;
    }

    match map.get_mut("type") {
        Some(Value::String(schema_type)) => {
            let schema_type = std::mem::take(schema_type);
            map.insert("type".to_owned(), serde_json::json!([schema_type, "null"]));
        }
        Some(Value::Array(types)) => {
            if !types.iter().any(|schema_type| schema_type == "null") {
                types.push(Value::String("null".to_owned()));
            }
        }
        _ => {
            let mut schema = std::mem::take(map);
            let annotations: Vec<String> = schema
                .keys()
                .filter(|key| *key == "readOnly" || *key == "writeOnly" || key.starts_with("x-"))
                .cloned()
                .collect();
            for key in annotations {
                if let Some(value) = schema.remove(&key) {
                    map.insert(key, value);
                }
            }
            map.insert(
                "anyOf".to_owned(),
                serde_json::json!([schema, { "type": "null" }]),
            );
        }
    }
}
