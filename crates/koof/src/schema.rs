//! Schema helpers for CRDs derived from generated types.

use serde_json::{Map, Value};

/// `#[schemars(transform = koof::schema::flatten_unions)]`.
///
/// A structural CRD schema allows no types or descriptions inside `oneOf` /
/// `anyOf`, and kube's rewriter only merges variants whose shared properties are
/// identical — which tagged unions (DNS records keyed by `type`), scalar unions
/// (`integer | string` ports) and mixed unions never satisfy. Every union is
/// replaced by one schema:
/// - object variants are merged: identical properties kept, conflicting ones
///   relaxed, `required` = intersection;
/// - scalars of one type become that type (enums unioned, `nullable` kept);
/// - `integer | string` becomes `x-kubernetes-int-or-string`;
/// - anything else becomes `x-kubernetes-preserve-unknown-fields` (validated by the
///   provider API instead of the API server).
pub fn flatten_unions(schema: &mut schemars::Schema) {
    if let Some(node) = schema.as_object_mut() {
        flatten(node);
    }
}

fn flatten(node: &mut Map<String, Value>) {
    for child in node.values_mut() {
        visit(child);
    }

    for keyword in ["oneOf", "anyOf"] {
        let Some(Value::Array(variants)) = node.remove(keyword) else {
            continue;
        };
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

/// Non-object (or mixed) union → one schema at `node`, per the rules above.
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

/// Replaces `existing` with what it and `incoming` agree on.
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

/// A CEL rule for `x-kubernetes-validations`, placed at `field` (dot path of
/// properties below the transformed schema; empty for its root).
#[derive(Debug)]
pub struct Validation {
    pub rule: &'static str,
    pub message: &'static str,
    pub field: &'static str,
}

/// Adds `validations` as `x-kubernetes-validations` to the schema nodes they name.
/// Run after [`flatten_unions`], so that union properties exist. A rule whose
/// field is not in the schema is skipped with a warning (the CRD would otherwise
/// fail in a less obvious way).
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
