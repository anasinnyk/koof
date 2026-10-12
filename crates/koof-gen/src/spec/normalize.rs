use super::schema_children_mut;
use serde_json::{Map, Value};

pub fn normalize_nullable(value: &mut Value) {
    match value {
        Value::Object(map) => normalize_object(map),
        Value::Array(items) => items.iter_mut().for_each(normalize_nullable),
        _ => {}
    }
}

pub fn relax_read_only_required(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for child in schema_children_mut(map) {
                relax_read_only_required(child);
            }
            let read_only: Vec<String> = map
                .get("properties")
                .and_then(Value::as_object)
                .into_iter()
                .flatten()
                .filter(|(_, property)| is_read_only(property))
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
        Value::Array(items) => items.iter_mut().for_each(relax_read_only_required),
        _ => {}
    }
}

fn is_read_only(property: &Value) -> bool {
    property.get("readOnly") == Some(&Value::Bool(true))
        || property
            .get("allOf")
            .and_then(Value::as_array)
            .is_some_and(|parts| parts.iter().any(is_read_only))
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
            let schema = Value::Object(std::mem::take(map));
            map.insert(
                "anyOf".to_owned(),
                serde_json::json!([schema, { "type": "null" }]),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn read_only_properties_are_not_required() {
        let mut value = json!({
            "type": "object",
            "properties": {
                "id": { "allOf": [{ "$ref": "#/components/schemas/Id" }, { "readOnly": true }] },
                "version": { "type": "string", "readOnly": true },
                "name": { "type": "string" }
            },
            "required": ["id", "version", "name"]
        });
        relax_read_only_required(&mut value);
        assert_eq!(value["required"], json!(["name"]));
    }

    fn normalized(mut value: Value) -> Value {
        normalize_nullable(&mut value);
        value
    }

    #[test]
    fn typed_schema_gets_null_type() {
        assert_eq!(
            normalized(json!({ "type": "string", "nullable": true })),
            json!({ "type": ["string", "null"] })
        );
    }

    #[test]
    fn untyped_schema_is_wrapped_in_any_of() {
        assert_eq!(
            normalized(json!({ "$ref": "#/components/schemas/A", "nullable": true })),
            json!({ "anyOf": [{ "$ref": "#/components/schemas/A" }, { "type": "null" }] })
        );
    }

    #[test]
    fn nullable_false_is_dropped() {
        assert_eq!(
            normalized(json!({ "type": "integer", "nullable": false })),
            json!({ "type": "integer" })
        );
    }

    #[test]
    fn nested_schemas_are_normalized() {
        assert_eq!(
            normalized(json!({
                "type": "object",
                "properties": { "a": { "type": "boolean", "nullable": true } }
            })),
            json!({
                "type": "object",
                "properties": { "a": { "type": ["boolean", "null"] } }
            })
        );
    }

    #[test]
    fn property_named_nullable_and_examples_are_untouched() {
        let value = json!({
            "type": "object",
            "properties": { "nullable": { "type": "boolean" } },
            "example": { "nullable": true, "type": "x" }
        });
        assert_eq!(normalized(value.clone()), value);
    }

    #[test]
    fn non_string_enums_move_to_the_description() {
        let mut value = json!({
            "type": "object",
            "properties": {
                "port": { "type": "integer", "enum": [80, 443], "description": "Port." },
                "mode": { "type": "string", "enum": ["a", "b"] }
            }
        });
        drop_non_string_enums(&mut value);
        assert_eq!(
            value,
            json!({
                "type": "object",
                "properties": {
                    "port": { "type": "integer", "description": "Port. Allowed values: 80, 443." },
                    "mode": { "type": "string", "enum": ["a", "b"] }
                }
            })
        );
    }
}
