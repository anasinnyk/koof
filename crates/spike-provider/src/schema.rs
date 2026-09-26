//! Prototype of a generic runtime helper: make schemars output of tagged unions
//! acceptable for kube's structural rewriter, which requires identical schemas for a
//! property that appears in several `oneOf`/`anyOf` variants.

use serde_json::{Map, Value};

/// `#[schemars(transform = flatten_object_unions)]`: merges object variants of
/// `oneOf`/`anyOf` into the node itself. Conflicting property schemas are relaxed
/// (enums unioned, otherwise only `type` kept); required = intersection of variants.
pub fn flatten_object_unions(schema: &mut schemars::Schema) {
    if let Some(object) = schema.as_object_mut() {
        flatten(object);
    }
}

fn flatten(node: &mut Map<String, Value>) {
    for value in node.values_mut() {
        visit(value);
    }
    for key in ["oneOf", "anyOf"] {
        let Some(Value::Array(variants)) = node.get(key) else { continue };
        let objects: Vec<&Map<String, Value>> = variants
            .iter()
            .filter_map(Value::as_object)
            .filter(|v| v.contains_key("properties"))
            .collect();
        if objects.len() != variants.len() {
            continue;
        }
        let mut properties = node
            .get("properties")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        let mut required: Option<Vec<Value>> = None;
        for variant in &objects {
            if let Some(props) = variant.get("properties").and_then(Value::as_object) {
                for (name, schema) in props {
                    match properties.get_mut(name) {
                        None => {
                            properties.insert(name.clone(), schema.clone());
                        }
                        Some(existing) if existing == schema => {}
                        Some(existing) => relax(existing, schema),
                    }
                }
            }
            let variant_required: Vec<Value> = variant
                .get("required")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default();
            required = Some(match required {
                None => variant_required,
                Some(current) => current.into_iter().filter(|r| variant_required.contains(r)).collect(),
            });
        }
        node.remove(key);
        node.insert("type".into(), "object".into());
        node.insert("properties".into(), Value::Object(properties));
        match required {
            Some(required) if !required.is_empty() => {
                node.insert("required".into(), Value::Array(required));
            }
            _ => {
                node.remove("required");
            }
        }
    }
}

fn visit(value: &mut Value) {
    match value {
        Value::Object(map) => flatten(map),
        Value::Array(items) => items.iter_mut().for_each(visit),
        _ => {}
    }
}

/// Two different schemas for the same property: keep what both agree on.
fn relax(existing: &mut Value, incoming: &Value) {
    let (Some(a), Some(b)) = (existing.as_object_mut(), incoming.as_object()) else { return };
    let same_type = a.get("type") == b.get("type");
    let enum_union = match (a.get("enum").and_then(Value::as_array), b.get("enum").and_then(Value::as_array)) {
        (Some(x), Some(y)) if same_type => {
            let mut values = x.clone();
            values.extend(y.iter().filter(|v| !x.contains(v)).cloned());
            Some(values)
        }
        _ => None,
    };
    let nullable = a.get("nullable") == Some(&Value::Bool(true)) || b.get("nullable") == Some(&Value::Bool(true));
    let mut relaxed = Map::new();
    if same_type {
        if let Some(t) = a.get("type") {
            relaxed.insert("type".into(), t.clone());
        }
        if let Some(values) = enum_union {
            relaxed.insert("enum".into(), Value::Array(values));
        }
        if a.get("type") == Some(&Value::String("array".into())) && a.get("items") == b.get("items") {
            if let Some(items) = a.get("items") {
                relaxed.insert("items".into(), items.clone());
            }
        }
        if a.get("type") == Some(&Value::String("object".into())) {
            relaxed.insert("x-kubernetes-preserve-unknown-fields".into(), Value::Bool(true));
        }
    } else {
        relaxed.insert("x-kubernetes-preserve-unknown-fields".into(), Value::Bool(true));
    }
    if nullable {
        relaxed.insert("nullable".into(), Value::Bool(true));
    }
    *a = relaxed;
}
