use super::{
    SpecError,
    extensions::{
        FieldExtension, FieldSelectionMode, LifecycleAction, ResourceExtension, kube_operations,
        kube_resources,
    },
    normalize::is_read_only,
    resolve_ref, schema_ref_name, schemas_mut, success_json_schema,
};
use crate::ir::{
    FieldPath, ItemReferenceIr, ListMapIr, ResourceFields, ResourceName, synthesized_schema,
};
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};

/// Adds the `ForProvider` and `AtProvider` schemas of every schema with
/// `x-kube-resource` to `components.schemas`, keyed by [`synthesized_schema`];
/// returns, per resource, the top-level `forProvider` fields that may be
/// late-initialized and the lists compared by key.
///
/// # Errors
///
/// Returns [`SpecError`] when an `x-kube-*` extension is malformed, an explicitly
/// selected field does not exist, the observe response path cannot be resolved, or a
/// synthesized name collides with an existing schema, or a `listMapKeys` field is
/// not a list of objects that require the keys.
pub fn synthesize(
    document: &mut Value,
) -> Result<BTreeMap<ResourceName, ResourceFields>, SpecError> {
    schemas_mut(document)?;
    let snapshot = document.clone();
    let schemas = snapshot
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .ok_or_else(|| SpecError::Missing("components.schemas".to_owned()))?;
    let observe_responses = observe_responses(&snapshot, schemas)?;
    let creatable = creatable_resources(&snapshot)?;

    let mut synthesized = Map::new();
    let mut roots: BTreeMap<ResourceName, String> = BTreeMap::new();
    let mut item_references = BTreeMap::new();
    let kinds: BTreeMap<ResourceName, String> = kube_resources(&snapshot)
        .filter_map(Result::ok)
        .map(|kube_resource| (kube_resource.extension.name, kube_resource.extension.kind))
        .collect();
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
        roots.insert(extension.name.clone(), resource.root("ForProvider"));
        item_references.insert(extension.name.clone(), extension.item_references.clone());
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

    for (name, root) in &roots {
        for (reference, item_reference) in &item_references[name] {
            add_item_reference(target, root, item_reference, &kinds).map_err(|message| {
                SpecError::Resource {
                    resource: name.0.clone(),
                    message: format!("itemReferences.{reference}: {message}"),
                }
            })?;
        }
    }

    roots
        .into_iter()
        .map(|(name, root)| {
            let filled = item_references[&name]
                .values()
                .filter_map(|reference| {
                    let (list, _) = reference.list_and_field()?;
                    Some((list.to_owned(), reference.into.clone()))
                })
                .collect();
            let fields = resource_fields(target, &root, &name, &filled)?;
            Ok((name, fields))
        })
        .collect()
}

/// Adds the reference field of `item_reference` to the items of its list in the
/// `forProvider` schema `root` (in each union variant that has the list), and makes
/// the field it fills optional.
fn add_item_reference(
    schemas: &mut Map<String, Value>,
    root: &str,
    item_reference: &ItemReferenceIr,
    kinds: &BTreeMap<ResourceName, String>,
) -> Result<(), String> {
    let Some((list, field)) = item_reference.list_and_field() else {
        return Ok(());
    };
    let kind = kinds
        .get(&item_reference.target)
        .ok_or_else(|| format!("unknown target resource {}", item_reference.target.0))?;
    let snapshot = schemas.clone();
    let objects: Vec<String> = match snapshot.get(root).and_then(|root| root.get("oneOf")) {
        Some(Value::Array(variants)) => variants.iter().filter_map(schema_ref_name).collect(),
        _ => vec![root.to_owned()],
    };
    let reference = json!({
        "type": "object",
        "description": format!(
            "A reference to an object of kind `{kind}` in the namespace of the resource \
             holding the reference; sets `{}`.",
            item_reference.into
        ),
        "required": ["name"],
        "properties": { "name": { "type": "string" } },
    });
    let mut found = false;
    for object in objects {
        let Some(property) = schemas
            .get_mut(&object)
            .and_then(|object| object.pointer_mut(&format!("/properties/{list}")))
        else {
            continue;
        };
        found = true;
        *property = resolve_ref(&snapshot, property).clone();
        let items = property
            .get_mut("items")
            .ok_or_else(|| format!("{list} is not a list"))?;
        *items = resolve_ref(&snapshot, items).clone();
        let items = items
            .as_object_mut()
            .filter(|items| {
                items
                    .get("properties")
                    .and_then(Value::as_object)
                    .is_some_and(|properties| properties.contains_key(&item_reference.into))
            })
            .ok_or_else(|| format!("the items of {list} have no field {}", item_reference.into))?;
        if let Some(Value::Array(required)) = items.get_mut("required") {
            required.retain(|name| name != item_reference.into.as_str());
        }
        if let Some(Value::Object(properties)) = items.get_mut("properties") {
            properties.insert(field.to_owned(), reference.clone());
        }
    }
    if found {
        Ok(())
    } else {
        Err(format!("{list} is not a forProvider field"))
    }
}

fn resource_fields(
    schemas: &Map<String, Value>,
    root_name: &str,
    resource: &ResourceName,
    filled: &BTreeSet<(String, String)>,
) -> Result<ResourceFields, SpecError> {
    let Some(root) = schemas.get(root_name) else {
        return Ok(ResourceFields::default());
    };
    let objects: Vec<&Value> = match root.get("oneOf").and_then(Value::as_array) {
        Some(variants) => variants
            .iter()
            .filter_map(|variant| schemas.get(&schema_ref_name(variant)?))
            .collect(),
        None => vec![root],
    };

    let mut fields = ResourceFields::default();
    for (name, property) in objects
        .iter()
        .filter_map(|object| object.get("properties").and_then(Value::as_object))
        .flatten()
    {
        let extensions = [property, resolve_ref(schemas, property)]
            .into_iter()
            .map(|schema| FieldExtension::of(schema, &format!("{root_name}.{name}")))
            .collect::<Result<Vec<_>, _>>()?;
        if extensions.iter().all(|field| field.late_initialize) {
            fields.late_initialize.push(name.clone());
        }
        if let Some(keys) = extensions
            .into_iter()
            .map(|field| field.list_map_keys)
            .find(|keys| !keys.is_empty())
        {
            let in_schema = check_list_map(schemas, property, &keys, |key| {
                filled.contains(&(name.clone(), key.to_owned()))
            })
            .map_err(|message| SpecError::Resource {
                resource: resource.0.clone(),
                message: format!("listMapKeys of {name}: {message}"),
            })?;
            fields
                .list_map_keys
                .insert(name.clone(), ListMapIr { keys, in_schema });
        }
    }
    fields.late_initialize.sort();
    fields.late_initialize.dedup();
    Ok(fields)
}

/// Kubernetes accepts `x-kubernetes-list-type: map` only on a list of objects whose
/// keys are required. A key that an item reference fills may be optional; the list
/// is then compared by key without declaring it a map. Returns whether every key is
/// required.
fn check_list_map(
    schemas: &Map<String, Value>,
    property: &Value,
    keys: &[String],
    filled: impl Fn(&str) -> bool,
) -> Result<bool, String> {
    let property = resolve_ref(schemas, property);
    let is_array = match property.get("type") {
        Some(Value::String(type_)) => type_ == "array",
        Some(Value::Array(types)) => types.iter().any(|type_| type_ == "array"),
        _ => false,
    };
    let items = property
        .get("items")
        .filter(|_| is_array)
        .map(|items| resolve_ref(schemas, items))
        .ok_or("not a list")?;
    let required = items
        .get("required")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default();
    let is_required = |key: &String| required.iter().any(|name| name == key.as_str());
    if let Some(key) = keys.iter().find(|key| {
        items.pointer(&format!("/properties/{key}")).is_none() || !(is_required(key) || filled(key))
    }) {
        return Err(format!("{key} is not a required property of the items"));
    }
    Ok(keys.iter().all(is_required))
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
    fn root(&self, role: &str) -> String {
        synthesized_schema(&self.extension.api_version, &self.extension.kind, role)
    }

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
        let root = self.root("ForProvider");
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
        let root = self.root("AtProvider");
        let selection = Selection {
            keep_required: false,
            writable_only: false,
        };
        let before: BTreeSet<String> = out.keys().cloned().collect();
        self.filtered(&root, "observe response", response, &fields, selection, out)?;
        for (name, schema) in out.iter_mut().filter(|(name, _)| !before.contains(*name)) {
            without_sensitive(self.schemas, schema, name, &mut BTreeSet::new())?;
        }
        Ok(())
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

        if out.contains_key(root) {
            return Err(self.error(&format!(
                "kind {} in {} is already used by another resource",
                self.extension.kind, self.extension.api_version
            )));
        }
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

/// Removes the properties marked `x-kube-field.sensitive` at any depth, inlining a
/// `$ref` whose target has one, so that secrets the API returns never reach the
/// status.
fn without_sensitive(
    schemas: &Map<String, Value>,
    schema: &mut Value,
    location: &str,
    visiting: &mut BTreeSet<String>,
) -> Result<(), SpecError> {
    if let Some(name) = schema_ref_name(schema) {
        if !has_sensitive(schemas, schema, location, &mut BTreeSet::new())?
            || !visiting.insert(name.clone())
        {
            return Ok(());
        }
        *schema = resolve_ref(schemas, schema).clone();
        without_sensitive(schemas, schema, location, visiting)?;
        visiting.remove(&name);
        return Ok(());
    }
    let Some(object) = schema.as_object_mut() else {
        return Ok(());
    };
    if let Some(properties) = object.get_mut("properties").and_then(Value::as_object_mut) {
        let mut sensitive = Vec::new();
        for (name, property) in properties.iter() {
            if is_sensitive(schemas, property, &format!("{location}.{name}"))? {
                sensitive.push(name.clone());
            }
        }
        for name in &sensitive {
            properties.remove(name);
        }
        for (name, property) in properties.iter_mut() {
            without_sensitive(schemas, property, &format!("{location}.{name}"), visiting)?;
        }
        if let Some(required) = object.get_mut("required").and_then(Value::as_array_mut) {
            required.retain(|name| !sensitive.iter().any(|sensitive| name == sensitive));
        }
    }
    for keyword in ["items", "additionalProperties"] {
        if let Some(nested) = object.get_mut(keyword) {
            without_sensitive(schemas, nested, location, visiting)?;
        }
    }
    for keyword in ["allOf", "anyOf", "oneOf"] {
        for variant in object
            .get_mut(keyword)
            .and_then(Value::as_array_mut)
            .into_iter()
            .flatten()
        {
            without_sensitive(schemas, variant, location, visiting)?;
        }
    }
    Ok(())
}

fn has_sensitive(
    schemas: &Map<String, Value>,
    schema: &Value,
    location: &str,
    visiting: &mut BTreeSet<String>,
) -> Result<bool, SpecError> {
    if let Some(name) = schema_ref_name(schema) {
        if !visiting.insert(name.clone()) {
            return Ok(false);
        }
        return has_sensitive(schemas, resolve_ref(schemas, schema), location, visiting);
    }
    for (name, property) in schema
        .get("properties")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
    {
        let location = format!("{location}.{name}");
        if is_sensitive(schemas, property, &location)?
            || has_sensitive(schemas, property, &location, visiting)?
        {
            return Ok(true);
        }
    }
    let nested = ["items", "additionalProperties"]
        .into_iter()
        .filter_map(|keyword| schema.get(keyword))
        .chain(
            ["allOf", "anyOf", "oneOf"]
                .into_iter()
                .filter_map(|keyword| schema.get(keyword).and_then(Value::as_array))
                .flatten(),
        );
    for schema in nested {
        if has_sensitive(schemas, schema, location, visiting)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn is_sensitive(
    schemas: &Map<String, Value>,
    property: &Value,
    location: &str,
) -> Result<bool, SpecError> {
    for schema in [property, resolve_ref(schemas, property)] {
        if FieldExtension::of(schema, location)?.sensitive {
            return Ok(true);
        }
    }
    Ok(false)
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
            if keyword == "allOf" {
                walk_properties(schemas, variant, visiting, visit);
            } else {
                walk_properties(schemas, variant, visiting, &mut |name, property, _| {
                    visit(name, property, false);
                });
            }
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
            for variant in variants
                .iter()
                .filter(|variant| variant.get("type").and_then(Value::as_str) != Some("null"))
            {
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
