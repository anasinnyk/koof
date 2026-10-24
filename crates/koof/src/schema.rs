use serde_json::{Map, Value};

pub fn flatten_unions(schema: &mut schemars::Schema) {
    if let Some(node) = schema.as_object_mut() {
        flatten(node);
        fill_untyped(node);
    }
}

const SHAPED: [&str; 7] = [
    "type",
    "properties",
    "items",
    "anyOf",
    "oneOf",
    "allOf",
    "x-kubernetes-int-or-string",
];

fn fill_untyped(node: &mut Map<String, Value>) {
    for key in ["items", "additionalProperties"] {
        if let Some(child) = node.get_mut(key) {
            fill_untyped_value(child);
        }
    }
    if let Some(Value::Object(properties)) = node.get_mut("properties") {
        properties.values_mut().for_each(fill_untyped_value);
    }
    if let Some(Value::Array(values)) = node.get_mut("enum")
        && values.iter().any(Value::is_null)
    {
        values.retain(|value| !value.is_null());
        if values.is_empty() {
            node.remove("enum");
        }
        node.insert("nullable".to_owned(), Value::Bool(true));
    }
    if !SHAPED.iter().any(|key| node.contains_key(*key))
        && !node.contains_key("x-kubernetes-preserve-unknown-fields")
    {
        node.insert(
            "x-kubernetes-preserve-unknown-fields".to_owned(),
            Value::Bool(true),
        );
    }
}

fn fill_untyped_value(value: &mut Value) {
    match value {
        Value::Bool(true) => {
            *value = serde_json::json!({ "x-kubernetes-preserve-unknown-fields": true });
        }
        Value::Object(node) => fill_untyped(node),
        _ => {}
    }
}

fn flatten(node: &mut Map<String, Value>) {
    for child in node.values_mut() {
        visit(child);
    }

    for keyword in ["oneOf", "anyOf"] {
        let Some(Value::Array(mut variants)) = node.remove(keyword) else {
            continue;
        };
        let count = variants.len();
        variants.retain(|variant| variant.get("type").and_then(Value::as_str) != Some("null"));
        if variants.len() < count {
            node.insert("nullable".to_owned(), Value::Bool(true));
        }
        if let [Value::Object(variant)] = variants.as_slice() {
            for (key, value) in variant {
                node.entry(key.clone()).or_insert_with(|| value.clone());
            }
            continue;
        }
        let objects: Vec<&Map<String, Value>> = variants
            .iter()
            .filter_map(Value::as_object)
            .filter(|variant| variant.contains_key("properties"))
            .collect();
        if !objects.is_empty() && objects.len() == variants.len() {
            merge_objects(node, &objects);
        } else if node.get("type").is_none() {
            merge_scalars(node, &variants);
        }
    }
}

fn merge_objects(node: &mut Map<String, Value>, objects: &[&Map<String, Value>]) {
    let mut properties = node
        .get("properties")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let mut required: Option<Vec<Value>> = None;

    for variant in objects {
        for (name, schema) in variant
            .get("properties")
            .and_then(Value::as_object)
            .into_iter()
            .flatten()
        {
            match properties.get_mut(name) {
                None => {
                    properties.insert(name.clone(), schema.clone());
                }
                Some(existing) if existing == schema => {}
                Some(existing) => relax(existing, schema),
            }
        }

        let variant_required = variant
            .get("required")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        required = Some(match required {
            None => variant_required,
            Some(current) => current
                .into_iter()
                .filter(|name| variant_required.contains(name))
                .collect(),
        });
    }

    node.insert("type".to_owned(), Value::String("object".to_owned()));
    node.insert("properties".to_owned(), Value::Object(properties));
    match required {
        Some(required) if !required.is_empty() => {
            node.insert("required".to_owned(), Value::Array(required));
        }
        _ => {
            node.remove("required");
        }
    }
}

fn merge_scalars(node: &mut Map<String, Value>, variants: &[Value]) {
    let mut types = std::collections::BTreeSet::new();
    let mut nullable = false;
    let mut mixed = false;
    for variant in variants {
        match variant.get("type").and_then(Value::as_str) {
            Some("null") => nullable = true,
            Some(scalar @ ("string" | "integer" | "number" | "boolean")) => {
                types.insert(scalar);
            }
            None if variant.get("x-kubernetes-int-or-string") == Some(&Value::Bool(true)) => {
                types.extend(["integer", "string"]);
            }
            _ => mixed = true,
        }
        nullable |= variant.get("nullable") == Some(&Value::Bool(true));
    }

    if !mixed && types.len() == 1 {
        let scalar = types.iter().next().copied().unwrap_or("string");
        node.insert("type".to_owned(), Value::String(scalar.to_owned()));
        let enums: Option<Vec<&Vec<Value>>> = variants
            .iter()
            .filter(|variant| variant.get("type").and_then(Value::as_str) != Some("null"))
            .map(|variant| variant.get("enum").and_then(Value::as_array))
            .collect();
        if let Some(enums) = enums {
            let mut values: Vec<Value> = Vec::new();
            for value in enums.into_iter().flatten() {
                if !values.contains(value) {
                    values.push(value.clone());
                }
            }
            node.insert("enum".to_owned(), Value::Array(values));
        }
    } else if !mixed && types.iter().all(|t| matches!(*t, "integer" | "string")) && types.len() == 2
    {
        node.insert("x-kubernetes-int-or-string".to_owned(), Value::Bool(true));
    } else {
        node.insert(
            "x-kubernetes-preserve-unknown-fields".to_owned(),
            Value::Bool(true),
        );
    }
    if nullable {
        node.insert("nullable".to_owned(), Value::Bool(true));
    }
}

fn visit(value: &mut Value) {
    match value {
        Value::Object(node) => flatten(node),
        Value::Array(items) => items.iter_mut().for_each(visit),
        _ => {}
    }
}

fn relax(existing: &mut Value, incoming: &Value) {
    let (Some(left), Some(right)) = (existing.as_object(), incoming.as_object()) else {
        return;
    };
    let is_true = |node: &Map<String, Value>, key: &str| node.get(key) == Some(&Value::Bool(true));

    let mut relaxed = Map::new();
    let same_type = left.get("type").is_some() && left.get("type") == right.get("type");
    if same_type {
        if let Some(schema_type) = left.get("type") {
            relaxed.insert("type".to_owned(), schema_type.clone());
        }
        if let (Some(Value::Array(a)), Some(Value::Array(b))) =
            (left.get("enum"), right.get("enum"))
        {
            let mut values = a.clone();
            values.extend(b.iter().filter(|value| !a.contains(value)).cloned());
            relaxed.insert("enum".to_owned(), Value::Array(values));
        }
        match left.get("type").and_then(Value::as_str) {
            Some("array") if left.get("items") == right.get("items") => {
                if let Some(items) = left.get("items") {
                    relaxed.insert("items".to_owned(), items.clone());
                }
            }
            Some("array") => {
                relaxed.insert(
                    "items".to_owned(),
                    serde_json::json!({ "x-kubernetes-preserve-unknown-fields": true }),
                );
            }
            Some("object") => {
                relaxed.insert(
                    "x-kubernetes-preserve-unknown-fields".to_owned(),
                    Value::Bool(true),
                );
            }
            _ => {}
        }
    } else {
        relaxed.insert(
            "x-kubernetes-preserve-unknown-fields".to_owned(),
            Value::Bool(true),
        );
    }
    if is_true(left, "nullable") || is_true(right, "nullable") {
        relaxed.insert("nullable".to_owned(), Value::Bool(true));
    }
    *existing = Value::Object(relaxed);
}

#[derive(Debug)]
pub struct Validation {
    pub rule: &'static str,
    pub message: &'static str,
    pub field: &'static str,
}

pub fn add_validations(schema: &mut schemars::Schema, validations: &[Validation]) {
    for validation in validations {
        let Some(node) = schema
            .as_object_mut()
            .and_then(|root| node_mut(root, validation.field))
        else {
            tracing::warn!(
                field = validation.field,
                rule = validation.rule,
                "validation field not found in the schema; rule skipped"
            );
            continue;
        };
        let rules = node
            .entry("x-kubernetes-validations")
            .or_insert_with(|| Value::Array(Vec::new()));
        if let Value::Array(rules) = rules {
            rules.push(serde_json::json!({
                "rule": validation.rule,
                "message": validation.message,
            }));
        }
    }
}

fn node_mut<'a>(
    root: &'a mut Map<String, Value>,
    path: &str,
) -> Option<&'a mut Map<String, Value>> {
    let mut current = root;
    for segment in path.split('.').filter(|segment| !segment.is_empty()) {
        current = current
            .get_mut("properties")?
            .get_mut(segment)?
            .as_object_mut()?;
    }
    Some(current)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use serde_json::json;

    fn flattened(schema: Value) -> Value {
        let mut schema = schemars::Schema::try_from(schema).expect("schema");
        flatten_unions(&mut schema);
        schema.to_value()
    }

    #[rstest]
    #[case::optional_value_becomes_nullable(
        json!({ "anyOf": [{ "type": "string" }, { "type": "null" }] }),
        json!({ "type": "string", "nullable": true })
    )]
    #[case::single_variant_is_inlined_keeping_the_description(
        json!({ "description": "Zone settings", "oneOf": [{ "type": "object", "properties": { "ttl": { "type": "integer" } } }] }),
        json!({ "description": "Zone settings", "type": "object", "properties": { "ttl": { "type": "integer" } } })
    )]
    #[case::object_variants_merge_their_properties(
        json!({ "oneOf": [
            { "type": "object", "properties": { "url": { "type": "string" } }, "required": ["url", "kind"] },
            { "type": "object", "properties": { "path": { "type": "string" } }, "required": ["kind"] },
        ] }),
        json!({
            "type": "object",
            "properties": { "url": { "type": "string" }, "path": { "type": "string" } },
            "required": ["kind"],
        })
    )]
    #[case::conflicting_property_types_accept_anything(
        json!({ "oneOf": [
            { "type": "object", "properties": { "value": { "type": "string" } } },
            { "type": "object", "properties": { "value": { "type": "integer" } } },
        ] }),
        json!({ "type": "object", "properties": { "value": { "x-kubernetes-preserve-unknown-fields": true } } })
    )]
    #[case::string_enums_are_combined(
        json!({ "anyOf": [{ "type": "string", "enum": ["A", "AAAA"] }, { "type": "string", "enum": ["CNAME"] }] }),
        json!({ "type": "string", "enum": ["A", "AAAA", "CNAME"] })
    )]
    #[case::integer_or_string(
        json!({ "anyOf": [{ "type": "integer" }, { "type": "string" }] }),
        json!({ "x-kubernetes-int-or-string": true })
    )]
    #[case::object_or_string_accepts_anything(
        json!({ "anyOf": [{ "type": "object", "properties": {} }, { "type": "string" }] }),
        json!({ "x-kubernetes-preserve-unknown-fields": true })
    )]
    #[case::unions_inside_properties_are_flattened_too(
        json!({ "type": "object", "properties": { "ttl": { "anyOf": [{ "type": "integer" }, { "type": "null" }] } } }),
        json!({ "type": "object", "properties": { "ttl": { "type": "integer", "nullable": true } } })
    )]
    fn unions_become_structural_schemas(#[case] schema: Value, #[case] structural: Value) {
        assert_eq!(flattened(schema), structural);
    }

    #[rstest]
    #[case::untyped_property(
        json!({ "type": "object", "properties": { "data": {} } }),
        json!({ "type": "object", "properties": { "data": { "x-kubernetes-preserve-unknown-fields": true } } })
    )]
    #[case::any_map_value(
        json!({ "type": "object", "additionalProperties": true }),
        json!({ "type": "object", "additionalProperties": { "x-kubernetes-preserve-unknown-fields": true } })
    )]
    #[case::null_in_an_enum_means_nullable(
        json!({ "type": "string", "enum": ["low", "high", null] }),
        json!({ "type": "string", "enum": ["low", "high"], "nullable": true })
    )]
    fn untyped_values_are_preserved_as_is(#[case] schema: Value, #[case] structural: Value) {
        assert_eq!(flattened(schema), structural);
    }

    fn with_validations(validations: &[Validation]) -> Value {
        let mut schema = schemars::Schema::try_from(json!({
            "type": "object",
            "properties": { "forProvider": { "type": "object", "properties": { "ttl": { "type": "integer" } } } },
        }))
        .expect("schema");
        add_validations(&mut schema, validations);
        schema.to_value()
    }

    #[test]
    fn validations_are_attached_to_their_field() {
        let schema = with_validations(&[Validation {
            rule: "self.ttl >= 30",
            message: "ttl must be at least 30",
            field: "forProvider",
        }]);
        assert_eq!(
            schema.pointer("/properties/forProvider/x-kubernetes-validations"),
            Some(&json!([{ "rule": "self.ttl >= 30", "message": "ttl must be at least 30" }]))
        );
    }

    #[test]
    fn validation_of_an_unknown_field_is_skipped() {
        let schema = with_validations(&[Validation {
            rule: "self == oldSelf",
            message: "name is immutable",
            field: "forProvider.name",
        }]);
        assert_eq!(schema, with_validations(&[]));
    }
}
