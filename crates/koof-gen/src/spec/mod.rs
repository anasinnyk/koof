//! Preprocessing of an `OpenAPI` document (overlays already applied) before code
//! generation: keep only what `x-kube-*` needs, translate the `OpenAPI` 3.0 schema
//! dialect to JSON Schema, and synthesize per-resource `forProvider` / `atProvider`
//! schemas. Everything works on `serde_json::Value`, so schemas reach the type
//! generator unchanged apart from these steps.

mod bodies;
pub mod extensions;
mod normalize;
mod prune;
mod synthesize;

pub use bodies::hoist_operation_bodies;
pub use normalize::{drop_non_string_enums, normalize_nullable};
pub use prune::prune;
pub use synthesize::synthesize;

use crate::ir::ResourceName;
use serde_json::{Map, Value};
use std::collections::BTreeMap;

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

/// The schema set for type generation, and what was learned building it.
#[derive(Debug)]
pub struct CodegenSpec {
    /// The document with JSON Schema dialect and synthesized `forProvider` /
    /// `atProvider` schemas; `components.schemas` is the input of type generation.
    pub document: Value,
    /// Per resource: top-level `forProvider` fields that may be late-initialized.
    pub late_initialize: BTreeMap<ResourceName, Vec<String>>,
}

/// Turns a [`prepare`]d document into the schema set for type generation:
/// [`normalize_nullable`], [`synthesize`], then [`drop_non_string_enums`].
///
/// # Errors
///
/// Returns [`SpecError`] when an `x-kube-*` extension is malformed or a resource's
/// field selection cannot be satisfied by its schemas.
pub fn for_codegen(prepared: &Value) -> Result<CodegenSpec, SpecError> {
    let mut document = prepared.clone();
    normalize_nullable(&mut document);
    let late_initialize = synthesize(&mut document)?;
    schemas_mut(&mut document)?
        .values_mut()
        .for_each(drop_non_string_enums);
    Ok(CodegenSpec {
        document,
        late_initialize,
    })
}

/// `components.schemas` of `document`, created when missing (a fully
/// dereferenced document may have no component schemas left after pruning).
///
/// # Errors
///
/// Returns [`SpecError::Missing`] when the document or `components` is not an object.
fn schemas_mut(document: &mut Value) -> Result<&mut Map<String, Value>, SpecError> {
    document
        .as_object_mut()
        .ok_or_else(|| SpecError::Missing("document".to_owned()))?
        .entry("components")
        .or_insert_with(|| Value::Object(Map::new()))
        .as_object_mut()
        .ok_or_else(|| SpecError::Missing("components".to_owned()))?
        .entry("schemas")
        .or_insert_with(|| Value::Object(Map::new()))
        .as_object_mut()
        .ok_or_else(|| SpecError::Missing("components.schemas".to_owned()))
}

/// The JSON media type of a `content` map: `application/json`, else any `*+json`.
fn json_media(content: &Map<String, Value>) -> Option<&Value> {
    content.get("application/json").or_else(|| {
        content
            .iter()
            .find(|(name, _)| name.ends_with("+json"))
            .map(|(_, media)| media)
    })
}

/// Mutable counterpart of [`json_media`].
fn json_media_mut(content: &mut Map<String, Value>) -> Option<&mut Value> {
    let key = if content.contains_key("application/json") {
        "application/json".to_owned()
    } else {
        content.keys().find(|name| name.ends_with("+json"))?.clone()
    };
    content.get_mut(&key)
}

/// Schema of the JSON content of a request body or response.
fn json_schema(container: &Value) -> Option<&Value> {
    json_media(container.get("content")?.as_object()?)?.get("schema")
}

/// Successful (`2xx`) statuses of a `responses` map, in order.
fn success_statuses(responses: &Map<String, Value>) -> Vec<String> {
    let mut statuses: Vec<String> = responses
        .keys()
        .filter(|status| status.starts_with('2'))
        .cloned()
        .collect();
    statuses.sort();
    statuses
}

/// JSON schema of the first successful response of `operation` that has one,
/// following a `$ref` to `components.responses`.
fn success_json_schema<'a>(document: &'a Value, operation: &'a Value) -> Option<&'a Value> {
    let responses = operation.get("responses")?.as_object()?;
    success_statuses(responses).into_iter().find_map(|status| {
        let response = &responses[&status];
        let response = response
            .get("$ref")
            .and_then(Value::as_str)
            .and_then(|reference| reference.strip_prefix('#'))
            .map_or(Some(response), |pointer| document.pointer(pointer))?;
        json_schema(response)
    })
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

/// The component schema `schema` refers to, or `schema` itself.
fn resolve_ref<'a>(schemas: &'a Map<String, Value>, schema: &'a Value) -> &'a Value {
    schema_ref_name(schema)
        .and_then(|name| schemas.get(&name))
        .unwrap_or(schema)
}

/// Component schema name of the JSON request body of `operation`, if it is a
/// `$ref` (a body that is itself a `$ref` has none).
#[must_use]
pub fn request_schema_name(operation: &Value) -> Option<String> {
    let body = operation.get("requestBody")?;
    if body.get("$ref").is_some() {
        return None;
    }
    schema_ref_name(json_schema(body)?)
}

/// Component schema name of the first successful JSON response of `operation`,
/// if it is a `$ref`.
#[must_use]
pub fn response_schema_name(document: &Value, operation: &Value) -> Option<String> {
    schema_ref_name(success_json_schema(document, operation)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepare_and_for_codegen_run_all_steps() -> Result<(), Box<dyn std::error::Error>> {
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
        apiVersion: test.example.io/v1
        kind: Thing
        fields: { mode: explicit, forProvider: [name], atProvider: [id, name] }
      type: object
      properties:
        id: { type: string, readOnly: true }
        name: { type: string, nullable: true }
    Unused: { type: string }
",
        )?;

        let processed = for_codegen(&prepare(&document)?)?;
        let schemas = &processed.document["components"]["schemas"];

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
