//! Preprocessing of an `OpenAPI` document (overlays already applied) before code
//! generation: keep only what `x-kube-*` needs, translate the `OpenAPI` 3.0 schema
//! dialect to JSON Schema, and synthesize per-resource `forProvider` / `atProvider`
//! schemas. Everything works on `serde_json::Value`, so schemas reach the type
//! generator unchanged apart from these steps.

mod bodies;
mod normalize;
mod prune;
mod synthesize;

pub use bodies::hoist_operation_bodies;
pub use normalize::normalize_nullable;
pub use prune::prune;
pub use synthesize::synthesize;

use serde_json::{Map, Value};

#[derive(Debug, thiserror::Error)]
pub enum SpecError {
    #[error("missing object at {0}")]
    Missing(String),

    #[error("invalid {extension} at {location}: {source}")]
    Extension {
        extension: &'static str,
        location: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("resource {resource}: {message}")]
    Resource { resource: String, message: String },
}

/// Steps that keep the document valid `OpenAPI` 3.0: [`prune`] and
/// [`hoist_operation_bodies`].
///
/// # Errors
///
/// Returns [`SpecError`] when the document lacks `paths` / `components`, an
/// `x-kube-operation` is malformed, or a hoisted name collides.
pub fn prepare(document: &Value) -> Result<Value, SpecError> {
    let mut document = prune(document)?;
    hoist_operation_bodies(&mut document)?;
    Ok(document)
}

/// Turns a [`prepare`]d document into the schema set for type generation:
/// [`normalize_nullable`] then [`synthesize`].
///
/// # Errors
///
/// Returns [`SpecError`] when an `x-kube-*` extension is malformed or a resource's
/// field selection cannot be satisfied by its schemas.
pub fn for_codegen(prepared: &Value) -> Result<Value, SpecError> {
    let mut document = prepared.clone();
    normalize_nullable(&mut document);
    synthesize(&mut document)?;
    Ok(document)
}

/// [`prepare`] followed by [`for_codegen`].
///
/// # Errors
///
/// See [`prepare`] and [`for_codegen`].
pub fn preprocess(document: &Value) -> Result<Value, SpecError> {
    for_codegen(&prepare(document)?)
}

/// The JSON media type of a `content` map: `application/json`, else any `*+json`.
fn json_media_mut(content: &mut Map<String, Value>) -> Option<&mut Value> {
    let key = if content.contains_key("application/json") {
        "application/json".to_owned()
    } else {
        content.keys().find(|name| name.ends_with("+json"))?.clone()
    };
    content.get_mut(&key)
}

/// Keys whose values are data, not schemas; walkers must not descend into them.
const DATA_KEYWORDS: [&str; 5] = ["example", "examples", "default", "enum", "const"];

/// Keys whose values map arbitrary names to schemas (or schema containers). Their
/// entries are always walked, even when a name equals a data keyword (a property
/// called `default`, a `default` response).
const NAME_MAPS: [&str; 5] = [
    "properties",
    "patternProperties",
    "$defs",
    "definitions",
    "responses",
];

/// Children of `map` that may contain schemas.
fn schema_children(map: &Map<String, Value>) -> impl Iterator<Item = &Value> {
    map.iter().flat_map(|(key, child)| {
        let entries: Box<dyn Iterator<Item = &Value>> = match child {
            Value::Object(entries) if NAME_MAPS.contains(&key.as_str()) => {
                Box::new(entries.values())
            }
            _ if DATA_KEYWORDS.contains(&key.as_str()) => Box::new(std::iter::empty()),
            _ => Box::new(std::iter::once(child)),
        };
        entries
    })
}

/// Mutable counterpart of [`schema_children`].
fn schema_children_mut(map: &mut Map<String, Value>) -> Vec<&mut Value> {
    let mut children = Vec::new();
    for (key, child) in map.iter_mut() {
        if NAME_MAPS.contains(&key.as_str()) && child.is_object() {
            if let Value::Object(entries) = child {
                children.extend(entries.values_mut());
            }
        } else if !DATA_KEYWORDS.contains(&key.as_str()) {
            children.push(child);
        }
    }
    children
}

const COMPONENT_SCHEMA_PREFIX: &str = "#/components/schemas/";

/// Decodes a JSON pointer segment (`~1` → `/`, `~0` → `~`).
fn decode_segment(segment: &str) -> String {
    segment.replace("~1", "/").replace("~0", "~")
}

/// Name of the component schema a `{"$ref": "#/components/schemas/<name>"}` points to.
fn schema_ref_name(schema: &Value) -> Option<String> {
    schema
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|reference| reference.strip_prefix(COMPONENT_SCHEMA_PREFIX))
        .map(decode_segment)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preprocess_runs_all_steps() -> Result<(), Box<dyn std::error::Error>> {
        let document: Value = serde_yaml::from_str(
            r"
openapi: 3.0.3
info: { title: test, version: '1' }
paths:
  /things/{id}:
    get:
      x-kube-operation: { resource: thing, lifecycle: observe, response: { path: result } }
      responses:
        '200':
          content:
            application/json:
              schema:
                type: object
                properties:
                  result: { $ref: '#/components/schemas/Thing' }
components:
  schemas:
    Thing:
      x-kube-resource:
        name: thing
        kind: Thing
        fields: { mode: explicit, forProvider: [name], atProvider: [id, name] }
      type: object
      properties:
        id: { type: string, readOnly: true }
        name: { type: string, nullable: true }
    Unused: { type: string }
",
        )?;

        let processed = preprocess(&document)?;
        let schemas = &processed["components"]["schemas"];

        assert!(schemas.get("Unused").is_none());
        assert_eq!(
            schemas["Thing"]["properties"]["name"]["type"],
            serde_json::json!(["string", "null"])
        );
        assert!(schemas.get("ThingForProvider").is_some());
        assert!(schemas.get("ThingAtProvider").is_some());
        Ok(())
    }
}
