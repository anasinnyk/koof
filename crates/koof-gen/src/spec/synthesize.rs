use super::{
    SpecError,
    extensions::{
        FieldExtension, FieldSelectionMode, LifecycleAction, ResourceExtension, kube_operations,
        kube_resources,
    },
    normalize::is_read_only,
    resolve_ref, schema_ref_name, schemas_mut, success_json_schema,
};
use crate::ir::{FieldPath, ResourceName};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};

/// Adds `<Kind>ForProvider` and `<Kind>AtProvider` schemas to `components.schemas`
/// for every schema with `x-kube-resource`; returns, per resource, the top-level
/// `forProvider` fields that may be late-initialized.
///
/// # Errors
///
/// Returns [`SpecError`] when an `x-kube-*` extension is malformed, an explicitly
/// selected field does not exist, the observe response path cannot be resolved, or a
/// synthesized name collides with an existing schema.
pub fn synthesize(document: &mut Value) -> Result<BTreeMap<ResourceName, Vec<String>>, SpecError> {
    schemas_mut(document)?;
    let snapshot = document.clone();
    let schemas = snapshot
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .ok_or_else(|| SpecError::Missing("components.schemas".to_owned()))?;
    let observe_responses = observe_responses(&snapshot, schemas)?;
    let creatable = creatable_resources(&snapshot)?;

    let mut synthesized = Map::new();
    let mut kinds = BTreeMap::new();
    for kube_resource in kube_resources(&snapshot) {
        let kube_resource = kube_resource?;
        let extension = &kube_resource.extension;
        let resource = Resource { schemas, extension };

        let creatable = creatable.contains(&extension.name);
        resource.for_provider(
            kube_resource.schema_name,
            kube_resource.schema,
            creatable,
            &mut synthesized,
        )?;
        match observe_responses.get(&extension.name) {
            Some(response) => resource.at_provider(response, &mut synthesized)?,
            None if extension.fields.at_provider.is_empty() => {}
            None => {
                return Err(resource.error(
                    "atProvider fields are selected but the observe operation has no JSON response",
                ));
            }
        }
        kinds.insert(extension.name.clone(), extension.kind.clone());
    }

    let target = schemas_mut(document)?;
    for (name, schema) in synthesized {
        if target.contains_key(&name) {
            return Err(SpecError::Resource {
                resource: name.clone(),
                message: format!("synthesized schema {name} collides with an existing schema"),
            });
        }
        target.insert(name, schema);
    }

    kinds
        .into_iter()
        .map(|(name, kind)| Ok((name, late_initialize_fields(target, &kind)?)))
        .collect()
}

fn late_initialize_fields(
    schemas: &Map<String, Value>,
    kind: &str,
) -> Result<Vec<String>, SpecError> {
    let Some(root) = schemas.get(&format!("{kind}ForProvider")) else {
        return Ok(Vec::new());
    };
    let objects: Vec<&Value> = match root.get("oneOf").and_then(Value::as_array) {
        Some(variants) => variants
            .iter()
            .filter_map(|variant| schemas.get(&schema_ref_name(variant)?))
            .collect(),
        None => vec![root],
    };

    let mut fields = Vec::new();
    for (name, property) in objects
        .iter()
        .filter_map(|object| object.get("properties").and_then(Value::as_object))
        .flatten()
    {
        let late_initialize = [property, resolve_ref(schemas, property)]
            .into_iter()
            .map(|schema| FieldExtension::of(schema, &format!("{kind}ForProvider.{name}")))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .all(|field| field.late_initialize);
        if late_initialize {
            fields.push(name.clone());
        }
    }
    fields.sort();
    fields.dedup();
    Ok(fields)
}

/// How [`Resource::filtered`] selects the definitions of a field.
#[derive(Clone, Copy)]
struct Selection {
    /// Keep a field required when every definition requires it.
    keep_required: bool,
    /// Skip read-only definitions (a field read-only in some union variants).
    writable_only: bool,
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
        keep_required: bool,
        out: &mut Map<String, Value>,
    ) -> Result<(), SpecError> {
        let fields = match self.extension.fields.mode {
            FieldSelectionMode::Explicit => names(&self.extension.fields.for_provider),
            FieldSelectionMode::All => {
                let mut fields = Vec::new();
                for name in self.all_property_names(schema) {
                    if !self.is_excluded_from_for_provider(schema, &name)? {
                        fields.push(name);
                    }
                }
                fields
            }
        };
        let root = format!("{}ForProvider", self.extension.kind);
        // With `all`, a field read-only in some variants of a union is kept only in
        // the variants where it is writable.
        let selection = Selection {
            keep_required,
            writable_only: self.extension.fields.mode == FieldSelectionMode::All,
        };
        self.filtered(&root, schema_name, schema, &fields, selection, out)
    }

    fn at_provider(&self, response: &Value, out: &mut Map<String, Value>) -> Result<(), SpecError> {
        let fields = match self.extension.fields.mode {
            FieldSelectionMode::Explicit => names(&self.extension.fields.at_provider),
            FieldSelectionMode::All => self.all_property_names(response),
        };
        let root = format!("{}AtProvider", self.extension.kind);
        let selection = Selection {
            keep_required: false,
            writable_only: false,
        };
        self.filtered(&root, "observe response", response, &fields, selection, out)
    }

    fn filtered(
        &self,
        root: &str,
        source: &str,
        schema: &Value,
        fields: &[String],
        selection: Selection,
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
            let object = self.filter_object(schema, fields, selection, &mut found);
            out.insert(root.to_owned(), object);
        } else {
            let mut refs = Vec::new();
            let mut used = BTreeSet::new();
            for (index, (variant_name, variant)) in variants.iter().enumerate() {
                let object = self.filter_object(variant, fields, selection, &mut found);
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
        selection: Selection,
        found: &mut BTreeSet<String>,
    ) -> Value {
        let mut properties = Map::new();
        let mut required = Vec::new();
        for field in fields {
            let mut definitions: Vec<(Value, bool)> = Vec::new();
            for definition in find_property(self.schemas, schema, field) {
                if selection.writable_only && is_read_only(self.schemas, &definition.0) {
                    continue;
                }
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
                    let definitions: Vec<Value> =
                        definitions.into_iter().map(|(schema, _)| schema).collect();
                    merge_scalars(self.schemas, &definitions)
                        .unwrap_or_else(|| json!({ "anyOf": definitions }))
                }
            };
            found.insert(field.clone());
            if selection.keep_required && is_required {
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

    /// Ignored anywhere, or read-only everywhere: a field read-only in some
    /// variants of a union stays in the variants where it is writable.
    fn is_excluded_from_for_provider(
        &self,
        schema: &Value,
        field: &str,
    ) -> Result<bool, SpecError> {
        let definitions = find_property(self.schemas, schema, field);
        for (property, _) in &definitions {
            let resolved = resolve_ref(self.schemas, property);
            for candidate in [property, resolved] {
                if FieldExtension::of(candidate, field)?.ignored {
                    return Ok(true);
                }
            }
        }
        Ok(!definitions.is_empty()
            && definitions
                .iter()
                .all(|(property, _)| is_read_only(self.schemas, property)))
    }

    fn error(&self, message: &str) -> SpecError {
        SpecError::Resource {
            resource: self.extension.name.0.clone(),
            message: message.to_owned(),
        }
    }
}

fn merge_scalars(schemas: &Map<String, Value>, definitions: &[Value]) -> Option<Value> {
    let resolved: Vec<&Value> = definitions
        .iter()
        .map(|definition| resolve_ref(schemas, definition))
        .collect();

    let mut scalar_type: Option<&str> = None;
    let mut nullable = false;
    for definition in &resolved {
        let (definition_type, definition_nullable) = match definition.get("type")? {
            Value::String(name) => (name.as_str(), false),
            Value::Array(types) => {
                let mut non_null = types
                    .iter()
                    .filter_map(Value::as_str)
                    .filter(|name| *name != "null");
                let name = non_null.next()?;
                if non_null.next().is_some() {
                    return None;
                }
                (name, types.len() > 1)
            }
            _ => return None,
        };
        if !matches!(definition_type, "string" | "integer" | "number" | "boolean") {
            return None;
        }
        match scalar_type {
            Some(existing) if existing != definition_type => return None,
            _ => scalar_type = Some(definition_type),
        }
        nullable |= definition_nullable;
    }

    let mut merged = Map::new();
    let scalar_type = scalar_type?;
    merged.insert(
        "type".to_owned(),
        if nullable {
            json!([scalar_type, "null"])
        } else {
            json!(scalar_type)
        },
    );
    if resolved
        .iter()
        .all(|definition| definition.get("enum").is_some())
    {
        let mut values: Vec<Value> = Vec::new();
        for value in resolved
            .iter()
            .filter_map(|definition| definition.get("enum").and_then(Value::as_array))
            .flatten()
        {
            if !values.contains(value) {
                values.push(value.clone());
            }
        }
        merged.insert("enum".to_owned(), Value::Array(values));
    }
    if let Some(description) = resolved
        .iter()
        .find_map(|definition| definition.get("description"))
    {
        merged.insert("description".to_owned(), description.clone());
    }
    Some(Value::Object(merged))
}

fn names(paths: &[FieldPath]) -> Vec<String> {
    paths.iter().map(|path| path.0.clone()).collect()
}

fn creatable_resources(document: &Value) -> Result<BTreeSet<ResourceName>, SpecError> {
    let mut resources = BTreeSet::new();
    for kube_operation in kube_operations(document) {
        let extension = kube_operation?.extension;
        if extension.lifecycle == LifecycleAction::Create {
            resources.insert(extension.resource);
        }
    }
    Ok(resources)
}

fn observe_responses(
    document: &Value,
    schemas: &Map<String, Value>,
) -> Result<BTreeMap<ResourceName, Value>, SpecError> {
    let mut responses = BTreeMap::new();
    for kube_operation in kube_operations(document) {
        let kube_operation = kube_operation?;
        let extension = &kube_operation.extension;
        if extension.lifecycle != LifecycleAction::Observe {
            continue;
        }
        let Some(mut schema) = success_json_schema(document, kube_operation.operation).cloned()
        else {
            continue;
        };
        if let Some(response) = &extension.response {
            for segment in response
                .path
                .split('.')
                .filter(|segment| !segment.is_empty())
            {
                let mut definitions: Vec<Value> = find_property(schemas, &schema, segment)
                    .into_iter()
                    .map(|(property, _)| property)
                    .collect();
                schema = match definitions.len() {
                    0 => {
                        return Err(SpecError::Resource {
                            resource: extension.resource.0.clone(),
                            message: format!(
                                "observe response at {} has no `{}`",
                                kube_operation.location(),
                                response.path
                            ),
                        });
                    }
                    1 => definitions.remove(0),
                    _ => json!({ "allOf": definitions }),
                };
            }
        }
        responses.insert(extension.resource.clone(), schema);
    }
    Ok(responses)
}

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
