use super::DATA_KEYWORDS;
use serde_json::{Map, Value};

/// Translates `OpenAPI` 3.0 `nullable: true` into JSON Schema:
/// `type: T` becomes `type: [T, "null"]`; an untyped schema (`$ref`, compositions)
/// becomes `anyOf: [<schema>, {type: "null"}]`. `nullable: false` is dropped.
pub fn normalize_nullable(value: &mut Value) {
    match value {
        Value::Object(map) => normalize_object(map),
        Value::Array(items) => items.iter_mut().for_each(normalize_nullable),
        _ => {}
    }
}

fn normalize_object(map: &mut Map<String, Value>) {
    for (key, child) in map.iter_mut() {
        if !DATA_KEYWORDS.contains(&key.as_str()) {
            normalize_nullable(child);
        }
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
}
