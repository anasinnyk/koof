use super::{
    SpecError, decode_segment,
    extensions::{HTTP_METHODS, OPERATION, RESOURCE},
    schema_children,
};
use serde_json::{Map, Value};
use std::collections::BTreeSet;

/// Returns a copy of the document that keeps only operations carrying
/// `x-kube-operation` and the components reachable from them, from schemas with
/// `x-kube-resource`, and all security schemes.
///
/// # Errors
///
/// Returns [`SpecError::Missing`] when `paths` or `components` is not an object.
pub fn prune(document: &Value) -> Result<Value, SpecError> {
    let paths = object(document, "paths")?;
    let components = object(document, "components")?;

    let kept_paths = paths
        .iter()
        .filter_map(|(path, item)| Some((path.clone(), keep_kube_operations(item.as_object()?)?)))
        .collect::<Map<_, _>>();

    let mut queue = Vec::new();
    for item in kept_paths.values() {
        collect_component_refs(item, &mut queue);
    }
    if let Some(schemas) = components.get("schemas").and_then(Value::as_object) {
        queue.extend(
            schemas
                .iter()
                .filter(|(_, schema)| schema.get(RESOURCE).is_some())
                .map(|(name, _)| ("schemas".to_owned(), name.clone())),
        );
    }
    if let Some(schemes) = components.get("securitySchemes").and_then(Value::as_object) {
        queue.extend(
            schemes
                .keys()
                .map(|name| ("securitySchemes".to_owned(), name.clone())),
        );
    }

    let mut kept_components = Map::new();
    let mut seen = BTreeSet::new();
    while let Some((kind, name)) = queue.pop() {
        if !seen.insert((kind.clone(), name.clone())) {
            continue;
        }
        let Some(component) = components.get(&kind).and_then(|section| section.get(&name)) else {
            continue;
        };
        collect_component_refs(component, &mut queue);
        if let Value::Object(section) = kept_components
            .entry(kind)
            .or_insert_with(|| Value::Object(Map::new()))
        {
            section.insert(name, component.clone());
        }
    }

    let mut pruned = document.clone();
    if let Value::Object(root) = &mut pruned {
        root.insert("paths".to_owned(), Value::Object(kept_paths));
        root.insert("components".to_owned(), Value::Object(kept_components));
    }
    Ok(pruned)
}

fn keep_kube_operations(item: &Map<String, Value>) -> Option<Value> {
    let is_kube_operation =
        |key: &str, value: &Value| HTTP_METHODS.contains(&key) && value.get(OPERATION).is_some();
    if !item
        .iter()
        .any(|(key, value)| is_kube_operation(key, value))
    {
        return None;
    }
    Some(Value::Object(
        item.iter()
            .filter(|(key, value)| {
                !HTTP_METHODS.contains(&key.as_str()) || is_kube_operation(key, value)
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect(),
    ))
}

fn collect_component_refs(value: &Value, out: &mut Vec<(String, String)>) {
    match value {
        Value::Object(map) => {
            if let Some((section, name)) = map
                .get("$ref")
                .and_then(Value::as_str)
                .and_then(|reference| reference.strip_prefix("#/components/"))
                .and_then(|rest| rest.split_once('/'))
            {
                out.push((section.to_owned(), decode_segment(name)));
            }
            for child in schema_children(map) {
                collect_component_refs(child, out);
            }
        }
        Value::Array(items) => items
            .iter()
            .for_each(|item| collect_component_refs(item, out)),
        _ => {}
    }
}

fn object<'a>(document: &'a Value, key: &str) -> Result<&'a Map<String, Value>, SpecError> {
    document
        .get(key)
        .and_then(Value::as_object)
        .ok_or_else(|| SpecError::Missing(key.to_owned()))
}
