use super::{DATA_KEYWORDS, SpecError, decode_segment};
use serde_json::{Map, Value};
use std::collections::BTreeSet;

const HTTP_METHODS: [&str; 8] = [
    "get", "put", "post", "delete", "options", "head", "patch", "trace",
];

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
                .filter(|(_, schema)| schema.get("x-kube-resource").is_some())
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

/// Path item with only its `x-kube-operation` operations (path-level keys such as
/// `parameters` stay); `None` when no operation is kept.
fn keep_kube_operations(item: &Map<String, Value>) -> Option<Value> {
    let is_kube_operation = |key: &str, value: &Value| {
        HTTP_METHODS.contains(&key) && value.get("x-kube-operation").is_some()
    };
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

/// Collects `(section, name)` for every `#/components/<section>/<name>` reference.
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
            for (key, child) in map {
                if !DATA_KEYWORDS.contains(&key.as_str()) {
                    collect_component_refs(child, out);
                }
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

#[cfg(test)]
mod tests {
    use super::*;

    const DOCUMENT: &str = r"
openapi: 3.0.3
info: { title: test, version: '1' }
paths:
  /things/{id}:
    parameters:
      - $ref: '#/components/parameters/Id'
    get:
      x-kube-operation: { resource: thing, lifecycle: observe }
      responses:
        '200': { $ref: '#/components/responses/ThingResponse' }
    delete:
      responses: { '204': { description: gone } }
  /other:
    get:
      responses:
        '200':
          content:
            application/json:
              schema: { $ref: '#/components/schemas/Other' }
components:
  parameters:
    Id: { name: id, in: path, required: true, schema: { type: string } }
  responses:
    ThingResponse:
      description: ok
      content:
        application/json:
          schema: { $ref: '#/components/schemas/Envelope' }
  schemas:
    Envelope:
      type: object
      properties:
        result: { $ref: '#/components/schemas/Thing' }
    Thing:
      x-kube-resource: { name: thing, kind: Thing }
      type: object
      properties:
        tags: { type: array, items: { $ref: '#/components/schemas/Tag' } }
      example: { $ref: '#/components/schemas/OnlyInExample' }
    Tag: { type: string }
    Other: { type: string }
    OnlyInExample: { type: string }
    Standalone:
      x-kube-resource: { name: standalone, kind: Standalone }
      type: object
  securitySchemes:
    token: { type: http, scheme: bearer }
";

    fn names(document: &Value, section: &str) -> Vec<String> {
        document["components"][section]
            .as_object()
            .map(|section| section.keys().cloned().collect())
            .unwrap_or_default()
    }

    #[test]
    fn keeps_only_kube_operations() -> Result<(), Box<dyn std::error::Error>> {
        let pruned = prune(&serde_yaml::from_str(DOCUMENT)?)?;
        let paths = pruned["paths"].as_object().ok_or("paths")?;

        assert_eq!(paths.keys().collect::<Vec<_>>(), ["/things/{id}"]);
        let item = &paths["/things/{id}"];
        assert!(item.get("get").is_some());
        assert!(item.get("delete").is_none());
        assert!(item.get("parameters").is_some());
        Ok(())
    }

    #[test]
    fn keeps_reachable_components_only() -> Result<(), Box<dyn std::error::Error>> {
        let pruned = prune(&serde_yaml::from_str(DOCUMENT)?)?;

        assert_eq!(
            names(&pruned, "schemas"),
            ["Envelope", "Standalone", "Tag", "Thing"]
        );
        assert_eq!(names(&pruned, "responses"), ["ThingResponse"]);
        assert_eq!(names(&pruned, "parameters"), ["Id"]);
        assert_eq!(names(&pruned, "securitySchemes"), ["token"]);
        Ok(())
    }

    #[test]
    fn missing_paths_is_an_error() {
        assert!(matches!(
            prune(&serde_json::json!({ "components": {} })),
            Err(SpecError::Missing(key)) if key == "paths"
        ));
    }
}
