use super::{SpecError, schema_ref_name};
use crate::ir::{FieldPath, FieldSelectionIr, FieldSelectionModeIr};
use serde::Deserialize;
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Deserialize)]
struct ResourceExtension {
    name: String,
    kind: String,
    #[serde(default)]
    fields: FieldSelectionIr,
}

#[derive(Debug, Deserialize)]
struct OperationExtension {
    resource: String,
    lifecycle: String,
    #[serde(default)]
    response: Option<ResponseExtension>,
}

/// Where the resource lives inside the observe response body, as a dot path
/// (e.g. `result` for `{ "success": true, "result": { ... } }`).
#[derive(Debug, Deserialize)]
struct ResponseExtension {
    path: String,
}

/// Adds `<Kind>ForProvider` and `<Kind>AtProvider` schemas to `components.schemas`
/// for every schema with `x-kube-resource`.
///
/// Fields come from the resource's `fields` selection: `explicit` takes the listed
/// names, `all` takes every property (minus `readOnly` and `x-kube-field.ignored`
/// for `forProvider`). Properties are looked up through `$ref`, `allOf`, `anyOf` and
/// `oneOf`. A resource schema that is a union of variants yields a `oneOf` of
/// per-variant filtered objects (`<Kind>ForProvider<Variant>`). `atProvider` is read
/// from the observe response at `x-kube-operation.response.path` and has no
/// required fields.
///
/// # Errors
///
/// Returns [`SpecError`] when an `x-kube-*` extension is malformed, an explicitly
/// selected field does not exist, the observe response path cannot be resolved, or a
/// synthesized name collides with an existing schema.
pub fn synthesize(document: &mut Value) -> Result<(), SpecError> {
    let schemas = document
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .cloned()
        .ok_or_else(|| SpecError::Missing("components.schemas".to_owned()))?;
    let observe_responses = observe_responses(document)?;

    let mut synthesized = Map::new();
    for (schema_name, schema) in &schemas {
        let Some(extension) = schema.get("x-kube-resource") else {
            continue;
        };
        let extension: ResourceExtension =
            serde_json::from_value(extension.clone()).map_err(|source| SpecError::Extension {
                extension: "x-kube-resource",
                location: format!("components.schemas.{schema_name}"),
                source,
            })?;
        let resource = Resource {
            schemas: &schemas,
            extension: &extension,
        };

        resource.for_provider(schema_name, schema, &mut synthesized)?;
        match observe_responses.get(&extension.name) {
            Some(response) => resource.at_provider(response, &mut synthesized)?,
            None if extension.fields.at_provider.is_empty() => {}
            None => {
                return Err(resource.error(
                    "atProvider fields are selected but the observe operation has no JSON response",
                ));
            }
        }
    }

    let target = document
        .pointer_mut("/components/schemas")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| SpecError::Missing("components.schemas".to_owned()))?;
    for (name, schema) in synthesized {
        if target.contains_key(&name) {
            return Err(SpecError::Resource {
                resource: name.clone(),
                message: format!("synthesized schema {name} collides with an existing schema"),
            });
        }
        target.insert(name, schema);
    }
    Ok(())
}

struct Resource<'a> {
    schemas: &'a Map<String, Value>,
    extension: &'a ResourceExtension,
}

impl Resource<'_> {
    fn for_provider(
        &self,
        schema_name: &str,
        schema: &Value,
        out: &mut Map<String, Value>,
    ) -> Result<(), SpecError> {
        let fields = match self.extension.fields.mode {
            FieldSelectionModeIr::Explicit => names(&self.extension.fields.for_provider),
            FieldSelectionModeIr::All => self
                .all_property_names(schema)
                .into_iter()
                .filter(|name| !self.is_excluded_from_for_provider(schema, name))
                .collect(),
        };
        let root = format!("{}ForProvider", self.extension.kind);
        self.filtered(&root, schema_name, schema, &fields, true, out)
    }

    fn at_provider(&self, response: &Value, out: &mut Map<String, Value>) -> Result<(), SpecError> {
        let fields = match self.extension.fields.mode {
            FieldSelectionModeIr::Explicit => names(&self.extension.fields.at_provider),
            FieldSelectionModeIr::All => self.all_property_names(response),
        };
        let root = format!("{}AtProvider", self.extension.kind);
        self.filtered(&root, "observe response", response, &fields, false, out)
    }

    /// Inserts `root` as one filtered object, or as a `oneOf` of filtered variants
    /// when `schema` is a union.
    fn filtered(
        &self,
        root: &str,
        source: &str,
        schema: &Value,
        fields: &[String],
        keep_required: bool,
        out: &mut Map<String, Value>,
    ) -> Result<(), SpecError> {
        let mut variants = Vec::new();
        collect_variants(
            self.schemas,
            schema,
            None,
            &mut variants,
            &mut BTreeSet::new(),
        );

        let mut found = BTreeSet::new();
        if variants.len() <= 1 {
            let object = self.filter_object(schema, fields, keep_required, &mut found);
            out.insert(root.to_owned(), object);
        } else {
            let mut refs = Vec::new();
            let mut used = BTreeSet::new();
            for (index, (variant_name, variant)) in variants.iter().enumerate() {
                let object = self.filter_object(variant, fields, keep_required, &mut found);
                let name = format!(
                    "{root}{}",
                    variant_suffix(variant_name.as_deref(), index, &mut used)
                );
                refs.push(json!({ "$ref": format!("#/components/schemas/{name}") }));
                out.insert(name, object);
            }
            out.insert(root.to_owned(), json!({ "oneOf": refs }));
        }

        let missing: Vec<&str> = fields
            .iter()
            .filter(|field| !found.contains(field.as_str()))
            .map(String::as_str)
            .collect();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(self.error(&format!("{root}: fields {missing:?} not found in {source}")))
        }
    }

    fn filter_object(
        &self,
        schema: &Value,
        fields: &[String],
        keep_required: bool,
        found: &mut BTreeSet<String>,
    ) -> Value {
        let mut properties = Map::new();
        let mut required = Vec::new();
        for field in fields {
            let mut definitions: Vec<(Value, bool)> = Vec::new();
            for definition in find_property(self.schemas, schema, field) {
                if !definitions
                    .iter()
                    .any(|(existing, _)| existing == &definition.0)
                {
                    definitions.push(definition);
                }
            }
            let is_required =
                !definitions.is_empty() && definitions.iter().all(|(_, required)| *required);
            let property = match definitions.len() {
                0 => continue,
                1 => definitions.remove(0).0,
                _ => {
                    json!({ "anyOf": definitions.into_iter().map(|(schema, _)| schema).collect::<Vec<_>>() })
                }
            };
            found.insert(field.clone());
            if keep_required && is_required {
                required.push(Value::String(field.clone()));
            }
            properties.insert(field.clone(), property);
        }

        let mut object = Map::new();
        object.insert("type".to_owned(), json!("object"));
        object.insert("properties".to_owned(), Value::Object(properties));
        if !required.is_empty() {
            object.insert("required".to_owned(), Value::Array(required));
        }
        Value::Object(object)
    }

    fn all_property_names(&self, schema: &Value) -> Vec<String> {
        let mut names = BTreeSet::new();
        walk_properties(
            self.schemas,
            schema,
            &mut BTreeSet::new(),
            &mut |name, _, _| {
                names.insert(name.to_owned());
            },
        );
        names.into_iter().collect()
    }

    fn is_excluded_from_for_provider(&self, schema: &Value, field: &str) -> bool {
        find_property(self.schemas, schema, field)
            .iter()
            .any(|(property, _)| {
                let resolved = schema_ref_name(property)
                    .and_then(|name| self.schemas.get(&name))
                    .unwrap_or(property);
                [property, resolved].iter().any(|candidate| {
                    candidate.get("readOnly") == Some(&Value::Bool(true))
                        || candidate.pointer("/x-kube-field/ignored") == Some(&Value::Bool(true))
                })
            })
    }

    fn error(&self, message: &str) -> SpecError {
        SpecError::Resource {
            resource: self.extension.name.clone(),
            message: message.to_owned(),
        }
    }
}

fn names(paths: &[FieldPath]) -> Vec<String> {
    paths.iter().map(|path| path.0.clone()).collect()
}

/// Observe response schemas per resource name, descended to `response.path`.
fn observe_responses(document: &Value) -> Result<BTreeMap<String, Value>, SpecError> {
    let schemas = document
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let mut responses = BTreeMap::new();
    let Some(paths) = document.get("paths").and_then(Value::as_object) else {
        return Ok(responses);
    };

    for (path, item) in paths {
        let Some(item) = item.as_object() else {
            continue;
        };
        for (method, operation) in item {
            let Some(extension) = operation.get("x-kube-operation") else {
                continue;
            };
            let location = format!("paths.{path}.{method}");
            let extension: OperationExtension =
                serde_json::from_value(extension.clone()).map_err(|source| {
                    SpecError::Extension {
                        extension: "x-kube-operation",
                        location: location.clone(),
                        source,
                    }
                })?;
            if extension.lifecycle != "observe" {
                continue;
            }
            let Some(mut schema) = success_json_schema(document, operation) else {
                continue;
            };
            if let Some(response) = &extension.response {
                for segment in response
                    .path
                    .split('.')
                    .filter(|segment| !segment.is_empty())
                {
                    // An envelope often declares the property twice (a generic
                    // `result` in the base schema, the concrete one in an `allOf`
                    // extension); all definitions apply, so combine them.
                    let mut definitions: Vec<Value> = find_property(&schemas, &schema, segment)
                        .into_iter()
                        .map(|(property, _)| property)
                        .collect();
                    schema = match definitions.len() {
                        0 => {
                            return Err(SpecError::Resource {
                                resource: extension.resource.clone(),
                                message: format!(
                                    "observe response at {location} has no `{}`",
                                    response.path
                                ),
                            });
                        }
                        1 => definitions.remove(0),
                        _ => json!({ "allOf": definitions }),
                    };
                }
            }
            responses.insert(extension.resource, schema);
        }
    }
    Ok(responses)
}

/// JSON schema of the first 2xx response, following `$ref` to `components.responses`.
fn success_json_schema(document: &Value, operation: &Value) -> Option<Value> {
    let responses = operation.get("responses")?.as_object()?;
    let mut statuses: Vec<&String> = responses
        .keys()
        .filter(|status| status.starts_with('2'))
        .collect();
    statuses.sort();
    statuses.into_iter().find_map(|status| {
        let mut response = &responses[status];
        if let Some(pointer) = response
            .get("$ref")
            .and_then(Value::as_str)
            .and_then(|reference| reference.strip_prefix('#'))
        {
            response = document.pointer(pointer)?;
        }
        let content = response.get("content")?.as_object()?;
        let media = content.get("application/json").or_else(|| {
            content
                .iter()
                .find(|(name, _)| name.ends_with("+json"))
                .map(|(_, media)| media)
        })?;
        media.get("schema").cloned()
    })
}

/// Every definition of property `field` reachable through `$ref` and
/// `allOf`/`anyOf`/`oneOf`, with whether the declaring node requires it.
fn find_property(schemas: &Map<String, Value>, schema: &Value, field: &str) -> Vec<(Value, bool)> {
    let mut found = Vec::new();
    walk_properties(
        schemas,
        schema,
        &mut BTreeSet::new(),
        &mut |name, property, required| {
            if name == field {
                found.push((property.clone(), required));
            }
        },
    );
    found
}

fn walk_properties(
    schemas: &Map<String, Value>,
    schema: &Value,
    visiting: &mut BTreeSet<String>,
    visit: &mut dyn FnMut(&str, &Value, bool),
) {
    if let Some(name) = schema_ref_name(schema) {
        if visiting.insert(name.clone()) {
            if let Some(target) = schemas.get(&name) {
                walk_properties(schemas, target, visiting, visit);
            }
            visiting.remove(&name);
        }
        return;
    }
    if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
        let required = schema.get("required").and_then(Value::as_array);
        for (name, property) in properties {
            let is_required =
                required.is_some_and(|required| required.iter().any(|item| item == name));
            visit(name, property, is_required);
        }
    }
    for keyword in ["allOf", "anyOf", "oneOf"] {
        for variant in schema
            .get(keyword)
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            walk_properties(schemas, variant, visiting, visit);
        }
    }
}

/// Flattens `anyOf`/`oneOf` (through `$ref`) into leaf variants, remembering the
/// component name each leaf came from.
fn collect_variants(
    schemas: &Map<String, Value>,
    schema: &Value,
    name: Option<String>,
    out: &mut Vec<(Option<String>, Value)>,
    visiting: &mut BTreeSet<String>,
) {
    if let Some(target) = schema_ref_name(schema) {
        if visiting.insert(target.clone()) {
            if let Some(resolved) = schemas.get(&target) {
                collect_variants(schemas, resolved, Some(target.clone()), out, visiting);
            }
            visiting.remove(&target);
        }
        return;
    }
    match schema
        .get("oneOf")
        .or_else(|| schema.get("anyOf"))
        .and_then(Value::as_array)
    {
        Some(variants) => {
            for variant in variants {
                collect_variants(schemas, variant, None, out, visiting);
            }
        }
        None => out.push((name, schema.clone())),
    }
}

/// `dns-records_ARecord` → `ARecord`; falls back to the full name, then to
/// `Variant<index>`, so suffixes stay unique.
fn variant_suffix(name: Option<&str>, index: usize, used: &mut BTreeSet<String>) -> String {
    let candidates = name
        .map(|name| {
            [
                name.rsplit('_').next().unwrap_or(name).to_owned(),
                name.to_owned(),
            ]
        })
        .into_iter()
        .flatten()
        .chain([format!("Variant{index}")]);
    for candidate in candidates {
        if used.insert(candidate.clone()) {
            return candidate;
        }
    }
    format!("Variant{index}")
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn document(resource_extension: &str) -> Result<Value, serde_yaml::Error> {
        serde_yaml::from_str(&format!(
            r"
openapi: 3.0.3
info: {{ title: test, version: '1' }}
paths:
  /records/{{id}}:
    get:
      x-kube-operation: {{ resource: record, lifecycle: observe, response: {{ path: result }} }}
      responses:
        '200':
          content:
            application/json:
              schema:
                type: object
                properties:
                  success: {{ type: boolean }}
                  result:
                    allOf:
                      - $ref: '#/components/schemas/Record'
                      - properties:
                          id: {{ type: string }}
                          created_on: {{ type: string, format: date-time }}
components:
  schemas:
    Shared:
      type: object
      required: [name]
      properties:
        name: {{ type: string }}
        comment: {{ type: string }}
        modified_on: {{ type: string, readOnly: true }}
        internal: {{ type: string, x-kube-field: {{ ignored: true }} }}
    Record:
      x-kube-resource: {resource_extension}
      allOf:
        - $ref: '#/components/schemas/Shared'
        - properties:
            ttl: {{ type: integer }}
"
        ))
    }

    fn schema<'a>(document: &'a Value, name: &str) -> &'a Value {
        &document["components"]["schemas"][name]
    }

    fn property_names(schema: &Value) -> Vec<String> {
        schema["properties"]
            .as_object()
            .map(|properties| properties.keys().cloned().collect())
            .unwrap_or_default()
    }

    #[test]
    fn explicit_selection_takes_listed_fields_through_all_of() -> TestResult {
        let mut document = document(
            "{ name: record, kind: Record, fields: { mode: explicit, forProvider: [name, ttl], atProvider: [id, name, created_on] } }",
        )?;
        synthesize(&mut document)?;

        let for_provider = schema(&document, "RecordForProvider");
        assert_eq!(property_names(for_provider), ["name", "ttl"]);
        assert_eq!(for_provider["required"], json!(["name"]));

        let at_provider = schema(&document, "RecordAtProvider");
        assert_eq!(property_names(at_provider), ["created_on", "id", "name"]);
        assert!(at_provider.get("required").is_none());
        Ok(())
    }

    #[test]
    fn all_selection_skips_read_only_and_ignored_for_provider_fields() -> TestResult {
        let mut document = document("{ name: record, kind: Record }")?;
        synthesize(&mut document)?;

        assert_eq!(
            property_names(schema(&document, "RecordForProvider")),
            ["comment", "name", "ttl"]
        );
        assert_eq!(
            property_names(schema(&document, "RecordAtProvider")),
            [
                "comment",
                "created_on",
                "id",
                "internal",
                "modified_on",
                "name",
                "ttl"
            ]
        );
        Ok(())
    }

    #[test]
    fn unknown_explicit_field_is_an_error() -> TestResult {
        let mut document = document(
            "{ name: record, kind: Record, fields: { mode: explicit, forProvider: [name, nope] } }",
        )?;
        let error = synthesize(&mut document).err().ok_or("expected an error")?;
        let message = error.to_string();
        assert!(
            message.contains("record") && message.contains("nope"),
            "{message}"
        );
        Ok(())
    }

    #[test]
    fn union_resource_becomes_one_of_filtered_variants() -> TestResult {
        let mut document: Value = serde_yaml::from_str(
            r"
openapi: 3.0.3
info: { title: test, version: '1' }
paths: {}
components:
  schemas:
    dns_ARecord:
      type: object
      required: [type, content]
      properties:
        type: { type: string, enum: [A] }
        content: { type: string, format: ipv4 }
        proxied: { type: boolean }
    dns_MXRecord:
      type: object
      properties:
        type: { type: string, enum: [MX] }
        content: { type: string }
        priority: { type: integer }
    Record:
      x-kube-resource:
        name: record
        kind: Record
        fields: { mode: explicit, forProvider: [type, content, priority] }
      anyOf:
        - $ref: '#/components/schemas/dns_ARecord'
        - $ref: '#/components/schemas/dns_MXRecord'
",
        )?;
        synthesize(&mut document)?;

        assert_eq!(
            schema(&document, "RecordForProvider"),
            &json!({ "oneOf": [
                { "$ref": "#/components/schemas/RecordForProviderARecord" },
                { "$ref": "#/components/schemas/RecordForProviderMXRecord" }
            ] })
        );
        let a = schema(&document, "RecordForProviderARecord");
        assert_eq!(property_names(a), ["content", "type"]);
        assert_eq!(a["required"], json!(["type", "content"]));
        assert_eq!(
            property_names(schema(&document, "RecordForProviderMXRecord")),
            ["content", "priority", "type"]
        );
        Ok(())
    }

    #[test]
    fn response_path_combines_all_definitions_of_the_envelope_property() -> TestResult {
        let mut document: Value = serde_yaml::from_str(
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
                allOf:
                  - $ref: '#/components/schemas/Envelope'
                  - properties:
                      result: { $ref: '#/components/schemas/Thing' }
components:
  schemas:
    Envelope:
      type: object
      properties:
        success: { type: boolean }
        result: { description: A result. }
    Thing:
      x-kube-resource:
        name: thing
        kind: Thing
        fields: { mode: explicit, forProvider: [name], atProvider: [id, name] }
      type: object
      properties:
        id: { type: string }
        name: { type: string }
",
        )?;
        synthesize(&mut document)?;

        assert_eq!(
            property_names(schema(&document, "ThingAtProvider")),
            ["id", "name"]
        );
        Ok(())
    }

    #[test]
    fn missing_response_path_is_an_error() -> TestResult {
        let mut document = document("{ name: record, kind: Record }")?;
        if let Some(extension) =
            document.pointer_mut("/paths/~1records~1{id}/get/x-kube-operation/response")
        {
            *extension = json!({ "path": "nope" });
        }
        let error = synthesize(&mut document).err().ok_or("expected an error")?;
        assert!(error.to_string().contains("nope"), "{error}");
        Ok(())
    }

    #[test]
    fn variant_suffixes_stay_unique() {
        let mut used = BTreeSet::new();
        assert_eq!(variant_suffix(Some("a_Record"), 0, &mut used), "Record");
        assert_eq!(variant_suffix(Some("b_Record"), 1, &mut used), "b_Record");
        assert_eq!(variant_suffix(None, 2, &mut used), "Variant2");
    }
}
