//! Preprocessing of an `OpenAPI` document (overlays already applied) before code
//! generation: keep only what `x-kube-*` needs, translate the `OpenAPI` 3.0 schema
//! dialect to JSON Schema, and synthesize per-resource `forProvider` / `atProvider`
//! schemas. Everything works on `serde_json::Value`, so schemas reach the type
//! generator unchanged apart from these steps.

mod normalize;
mod prune;
mod synthesize;

pub use normalize::normalize_nullable;
pub use prune::prune;
pub use synthesize::synthesize;

use serde_json::Value;

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

/// Runs [`prune`], [`normalize_nullable`] and [`synthesize`] in that order.
///
/// # Errors
///
/// Returns [`SpecError`] when the document lacks `paths` / `components`, an
/// `x-kube-*` extension is malformed, or a resource's field selection cannot be
/// satisfied by its schemas.
pub fn preprocess(document: &Value) -> Result<Value, SpecError> {
    let mut document = prune(document)?;
    normalize_nullable(&mut document);
    synthesize(&mut document)?;
    Ok(document)
}

/// Keys whose values are data, not schemas; walkers must not descend into them.
const DATA_KEYWORDS: [&str; 5] = ["example", "examples", "default", "enum", "const"];

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
