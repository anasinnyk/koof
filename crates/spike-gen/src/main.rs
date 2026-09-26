//! Phase 2 spike: overlays -> roas (parse, validate, 3.0 -> 3.1) -> prune -> typify.
//!
//! Usage: cargo run -p spike-gen -- <openapi> <output.rs> [overlay...]

use anyhow::{Context, Result, anyhow, bail};
use enumset::EnumSet;
use roas::validation::{Options, Validate};
use roas_overlay::{
    apply::{Apply, ApplyOptions},
    v1_1::Overlay,
};
use serde_json::{Map, Value};
use std::{collections::BTreeSet, fs, time::Instant};

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let openapi_path = args.next().context("missing <openapi>")?;
    let output_path = args.next().context("missing <output.rs>")?;
    let overlays: Vec<String> = args.collect();

    // 1. Load + overlays (unchanged from koof-gen).
    let t = Instant::now();
    let mut doc: Value = serde_yaml::from_str(&fs::read_to_string(&openapi_path)?)?;
    for path in &overlays {
        let overlay: Overlay = serde_yaml::from_str(&fs::read_to_string(path)?)?;
        overlay
            .apply(&mut doc, ApplyOptions::ErrorOnZeroMatch.into())
            .map_err(|e| anyhow!("overlay {path}: {e}"))?;
    }
    println!("[load+overlay] {:?}", t.elapsed());
    check_openapiv3(&doc);
    let t = Instant::now();
    let full_components = component_count(&doc);
    let mut doc = prune_document(&doc)?;
    println!(
        "[prune before roas] components {} -> {} in {:?}",
        full_components,
        component_count(&doc),
        t.elapsed()
    );
    let _ = count_extensions(&doc);

    // 2. Only normalization typify needs: OpenAPI 3.0 `nullable` -> JSON Schema.
    //    (roas dropped: its typed round-trip is lossy, see docs/refactor-plan.md)
    let mut doc31 = doc;
    if std::env::var_os("SKIP_NULLABLE").is_none() {
        normalize_nullable(&mut doc31);
    } else {
        println!("[nullable] normalizer skipped");
    }

    // 3. Schemas reachable from x-kube resources and x-kube operations.
    let schemas = doc31
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .context("components.schemas")?;
    let roots = prune_roots(&doc31)?;
    let reachable = closure(schemas, roots);
    println!(
        "[prune] {} of {} component schemas reachable",
        reachable.len(),
        schemas.len()
    );

    // 3b. Synthesize <Kind>ForProvider: a composition of variants becomes a oneOf of
    //     per-variant filtered objects; a plain object becomes one filtered object.
    let mut synthesized = Map::new();
    for name in &roots_resources(&doc31) {
        let schema = &schemas[name];
        let ext = &schema["x-kube-resource"];
        let kind = ext["kind"].as_str().context("kind")?;
        let fields: Vec<&str> = ext
            .pointer("/fields/forProvider")
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(Value::as_str).collect())
            .unwrap_or_default();

        let mut variants = Vec::new();
        collect_variants(schemas, schema, None, &mut variants, &mut BTreeSet::new());
        let root = format!("{kind}ForProvider");
        if variants.len() <= 1 {
            let object = filter_fields(schemas, schema, &fields);
            if object["properties"].as_object().map_or(0, Map::len) < fields.len() {
                let missing: Vec<_> = fields
                    .iter()
                    .filter(|f| object["properties"].get(**f).is_none())
                    .collect();
                bail!("{kind}: forProvider fields {missing:?} not found in {name}");
            }
            synthesized.insert(root, object);
        } else {
            let mut refs = Vec::new();
            let mut seen_fields = BTreeSet::new();
            for (variant_name, variant) in &variants {
                let object = filter_fields(schemas, variant, &fields);
                seen_fields.extend(
                    object["properties"]
                        .as_object()
                        .into_iter()
                        .flat_map(|p| p.keys().cloned()),
                );
                let short = variant_name.rsplit('_').next().unwrap_or(variant_name);
                let def = format!("{root}{short}");
                refs.push(serde_json::json!({"$ref": format!("#/components/schemas/{def}")}));
                synthesized.insert(def, object);
            }
            let missing: Vec<_> = fields
                .iter()
                .filter(|f| !seen_fields.contains(**f))
                .collect();
            if !missing.is_empty() {
                bail!("{kind}: forProvider fields {missing:?} not found in any variant of {name}");
            }
            println!("    {kind}: {} variants -> oneOf", variants.len());
            synthesized.insert(root, serde_json::json!({ "oneOf": refs }));
        }
    }
    println!("[synthesize] {} schemas", synthesized.len());

    // 4. typify, with JsonSchema: CRDs are derived by kube from these types.
    let mut settings = typify::TypeSpaceSettings::default();
    settings
        .with_struct_builder(false)
        .with_derive("schemars::JsonSchema".to_owned())
        .with_derive("PartialEq".to_owned());
    let defs = reachable
        .iter()
        .map(|name| (name.clone(), schemas[name].clone()))
        .chain(synthesized.clone())
        .map(|(name, value)| {
            let schema: schemars08::schema::Schema =
                serde_json::from_value(strip_extensions(value))
                    .with_context(|| format!("schemars parse {name}"))?;
            Ok((name, schema))
        })
        .collect::<Result<Vec<_>>>()?;
    let t = Instant::now();
    let mut space = typify::TypeSpace::new(&settings);
    space.add_ref_types(defs).context("typify add_ref_types")?;
    let file: syn::File = syn::parse2(space.to_stream())?;
    let code = prettyplease::unparse(&file);
    let manual = code.matches("impl ::schemars::JsonSchema for").count();
    println!(
        "[typify] {} types, {} lines, {manual} hand-written JsonSchema impl(s) in {:?}",
        space.iter_types().count(),
        code.lines().count(),
        t.elapsed()
    );
    fs::write(
        &output_path,
        format!("#![allow(clippy::all, clippy::pedantic)]\n{code}"),
    )?;
    println!("[write] {output_path}");
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct Extensions {
    resources: usize,
    operations: usize,
}

fn count_extensions(doc: &Value) -> Extensions {
    let resources = doc
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .map_or(0, |s| {
            s.values()
                .filter(|v| v.get("x-kube-resource").is_some())
                .count()
        });
    let operations = doc
        .get("paths")
        .and_then(Value::as_object)
        .map_or(0, |paths| {
            paths
                .values()
                .filter_map(Value::as_object)
                .flat_map(|item| item.values())
                .filter(|op| op.get("x-kube-operation").is_some())
                .count()
        });
    Extensions {
        resources,
        operations,
    }
}

/// Schema names referenced by x-kube resources and by request/response bodies of
/// x-kube operations (following `$ref` into components.requestBodies/responses).
fn prune_roots(doc: &Value) -> Result<BTreeSet<String>> {
    let mut roots = BTreeSet::new();
    if let Some(schemas) = doc
        .pointer("/components/schemas")
        .and_then(Value::as_object)
    {
        for (name, schema) in schemas {
            if schema.get("x-kube-resource").is_some() {
                roots.insert(name.clone());
            }
        }
    }
    let paths = doc
        .get("paths")
        .and_then(Value::as_object)
        .context("paths")?;
    for item in paths.values().filter_map(Value::as_object) {
        for op in item
            .values()
            .filter(|op| op.get("x-kube-operation").is_some())
        {
            let mut bodies = Vec::new();
            if let Some(body) = op.get("requestBody") {
                bodies.push(resolve(doc, body));
            }
            if let Some(responses) = op.get("responses").and_then(Value::as_object) {
                for (status, response) in responses {
                    if status.starts_with('2') {
                        bodies.push(resolve(doc, response));
                    }
                }
            }
            for body in bodies {
                let mut refs = BTreeSet::new();
                collect_refs(body.get("content").unwrap_or(&Value::Null), &mut refs);
                roots.extend(refs);
            }
        }
    }
    Ok(roots)
}

fn resolve<'a>(doc: &'a Value, value: &'a Value) -> &'a Value {
    match value.get("$ref").and_then(Value::as_str) {
        Some(r) => r
            .strip_prefix('#')
            .and_then(|p| doc.pointer(p))
            .unwrap_or(value),
        None => value,
    }
}

fn collect_refs(value: &Value, out: &mut BTreeSet<String>) {
    match value {
        Value::Object(map) => {
            if let Some(name) = map
                .get("$ref")
                .and_then(Value::as_str)
                .and_then(|r| r.strip_prefix("#/components/schemas/"))
            {
                out.insert(name.replace("~1", "/").replace("~0", "~"));
            }
            map.values().for_each(|v| collect_refs(v, out));
        }
        Value::Array(items) => items.iter().for_each(|v| collect_refs(v, out)),
        _ => {}
    }
}

fn closure(schemas: &Map<String, Value>, roots: BTreeSet<String>) -> BTreeSet<String> {
    let mut seen = BTreeSet::new();
    let mut queue: Vec<String> = roots.into_iter().collect();
    while let Some(name) = queue.pop() {
        if !seen.insert(name.clone()) {
            continue;
        }
        if let Some(schema) = schemas.get(&name) {
            let mut refs = BTreeSet::new();
            collect_refs(schema, &mut refs);
            queue.extend(refs.into_iter().filter(|r| !seen.contains(r)));
        }
    }
    seen
}

fn strip_extensions(value: Value) -> Value {
    match value {
        Value::Object(map) => Value::Object(
            map.into_iter()
                .filter(|(k, _)| !k.starts_with("x-"))
                .map(|(k, v)| (k, strip_extensions(v)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.into_iter().map(strip_extensions).collect()),
        other => other,
    }
}

fn stash_formats(value: &mut Value) -> usize {
    let mut count = 0;
    match value {
        Value::Object(map) => {
            let ty = map.get("type").and_then(Value::as_str).map(str::to_owned);
            let format = map.get("format").and_then(Value::as_str).map(str::to_owned);
            if let (Some(ty), Some(format)) = (ty, format) {
                let standard = match ty.as_str() {
                    "integer" => matches!(format.as_str(), "int32" | "int64"),
                    "number" => matches!(format.as_str(), "float" | "double"),
                    _ => true,
                };
                if !standard {
                    map.remove("format");
                    map.insert("x-koof-format".into(), Value::String(format));
                    count += 1;
                }
            }
            for v in map.values_mut() {
                count += stash_formats(v);
            }
        }
        Value::Array(items) => {
            for v in items {
                count += stash_formats(v);
            }
        }
        _ => {}
    }
    count
}

fn restore_formats(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if let Some(format) = map.remove("x-koof-format") {
                map.insert("format".into(), format);
            }
            map.values_mut().for_each(restore_formats);
        }
        Value::Array(items) => items.iter_mut().for_each(restore_formats),
        _ => {}
    }
}

fn component_count(doc: &Value) -> usize {
    doc.get("components")
        .and_then(Value::as_object)
        .map_or(0, |c| {
            c.values().filter_map(Value::as_object).map(Map::len).sum()
        })
}

/// Keeps only operations with x-kube-operation and every component reachable
/// from them or from x-kube-resource schemas (any `#/components/<kind>/<name>`).
fn prune_document(doc: &Value) -> Result<Value> {
    let mut out = doc.clone();
    let obj = out.as_object_mut().context("document")?;

    let mut paths = Map::new();
    for (path, item) in doc
        .get("paths")
        .and_then(Value::as_object)
        .context("paths")?
    {
        let Some(item) = item.as_object() else {
            continue;
        };
        let kept: Map<String, Value> = item
            .iter()
            .filter(|(key, op)| {
                !matches!(
                    key.as_str(),
                    "get" | "put" | "post" | "delete" | "patch" | "head" | "options" | "trace"
                ) || op.get("x-kube-operation").is_some()
            })
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        if kept
            .keys()
            .any(|k| matches!(k.as_str(), "get" | "put" | "post" | "delete" | "patch"))
        {
            paths.insert(path.clone(), Value::Object(kept));
        }
    }

    let components = doc
        .get("components")
        .and_then(Value::as_object)
        .context("components")?;
    let mut queue: Vec<(String, String)> = Vec::new();
    collect_component_refs(&Value::Object(paths.clone()), &mut queue);
    if let Some(schemas) = components.get("schemas").and_then(Value::as_object) {
        for (name, schema) in schemas {
            if schema.get("x-kube-resource").is_some() {
                queue.push(("schemas".into(), name.clone()));
            }
        }
    }
    if let Some(schemes) = components.get("securitySchemes").and_then(Value::as_object) {
        queue.extend(
            schemes
                .keys()
                .map(|n| ("securitySchemes".to_owned(), n.clone())),
        );
    }

    let mut kept: BTreeSet<(String, String)> = BTreeSet::new();
    while let Some(key) = queue.pop() {
        if !kept.insert(key.clone()) {
            continue;
        }
        if let Some(value) = components.get(&key.0).and_then(|c| c.get(&key.1)) {
            collect_component_refs(value, &mut queue);
        }
    }

    let mut pruned = Map::new();
    for (kind, name) in &kept {
        if let Some(value) = components.get(kind).and_then(|c| c.get(name)) {
            pruned
                .entry(kind.clone())
                .or_insert_with(|| Value::Object(Map::new()))
                .as_object_mut()
                .context("component kind")?
                .insert(name.clone(), value.clone());
        }
    }
    obj.insert("paths".into(), Value::Object(paths));
    obj.insert("components".into(), Value::Object(pruned));
    obj.remove("tags");
    Ok(out)
}

fn collect_component_refs(value: &Value, out: &mut Vec<(String, String)>) {
    match value {
        Value::Object(map) => {
            if let Some(rest) = map
                .get("$ref")
                .and_then(Value::as_str)
                .and_then(|r| r.strip_prefix("#/components/"))
                && let Some((kind, name)) = rest.split_once('/')
            {
                out.push((kind.to_owned(), name.replace("~1", "/").replace("~0", "~")));
            }
            map.values().for_each(|v| collect_component_refs(v, out));
        }
        Value::Array(items) => items.iter().for_each(|v| collect_component_refs(v, out)),
        _ => {}
    }
}

/// Parses each component and operation separately to list every roas rejection.
fn report_roas_failures(doc: &Value) {
    let mut failures = Vec::new();
    if let Some(schemas) = doc
        .pointer("/components/schemas")
        .and_then(Value::as_object)
    {
        for (name, schema) in schemas {
            if let Err(e) =
                serde_path_to_error::deserialize::<_, roas::v3_0::schema::Schema>(schema.clone())
            {
                failures.push(format!("components.schemas.{name}.{e}"));
            }
        }
    }
    for kind in ["requestBodies", "responses", "parameters"] {
        if let Some(items) = doc
            .pointer(&format!("/components/{kind}"))
            .and_then(Value::as_object)
        {
            for (name, item) in items {
                let result = match kind {
                    "requestBodies" => serde_path_to_error::deserialize::<
                        _,
                        roas::v3_0::request_body::RequestBody,
                    >(item.clone())
                    .map(|_| ()),
                    "responses" => serde_path_to_error::deserialize::<
                        _,
                        roas::v3_0::response::Response,
                    >(item.clone())
                    .map(|_| ()),
                    _ => serde_path_to_error::deserialize::<_, roas::v3_0::parameter::Parameter>(
                        item.clone(),
                    )
                    .map(|_| ()),
                };
                if let Err(e) = result {
                    failures.push(format!("components.{kind}.{name}.{e}"));
                }
            }
        }
    }
    if let Some(paths) = doc.get("paths").and_then(Value::as_object) {
        for (path, item) in paths {
            if let Err(e) =
                serde_path_to_error::deserialize::<_, roas::v3_0::path_item::PathItem>(item.clone())
            {
                failures.push(format!("paths.{path}.{e}"));
            }
        }
    }
    println!(
        "[roas failures] {} item(s) rejected (first error per item):",
        failures.len()
    );
    for f in &failures {
        println!("    {f}");
    }
}

/// Minimal 3.0 -> JSON Schema normalization: `nullable: true` -> `type: [T, "null"]`.
fn normalize_nullable(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if map.remove("nullable") == Some(Value::Bool(true)) {
                match map.get("type").cloned() {
                    Some(Value::String(ty)) => {
                        map.insert("type".into(), serde_json::json!([ty, "null"]));
                    }
                    Some(_) => {}
                    None => {
                        // untyped (allOf/oneOf/$ref): wrap as anyOf [self, null]
                        let inner = Value::Object(std::mem::take(map));
                        map.insert("anyOf".into(), serde_json::json!([inner, {"type": "null"}]));
                    }
                }
            }
            map.values_mut().for_each(normalize_nullable);
        }
        Value::Array(items) => items.iter_mut().for_each(normalize_nullable),
        _ => {}
    }
}

fn strip_key(schema: &schemars08::schema::Schema, key: &str) -> schemars08::schema::Schema {
    fn walk(value: Value, key: &str) -> Value {
        match value {
            Value::Object(map) => Value::Object(
                map.into_iter()
                    .filter(|(k, _)| k != key)
                    .map(|(k, v)| (k, walk(v, key)))
                    .collect(),
            ),
            Value::Array(items) => Value::Array(items.into_iter().map(|v| walk(v, key)).collect()),
            other => other,
        }
    }
    let value = serde_json::to_value(schema).unwrap_or(Value::Null);
    serde_json::from_value(walk(value, key)).unwrap_or_else(|_| schema.clone())
}

fn roots_resources(doc: &Value) -> Vec<String> {
    doc.pointer("/components/schemas")
        .and_then(Value::as_object)
        .map(|s| {
            s.iter()
                .filter(|(_, v)| v.get("x-kube-resource").is_some())
                .map(|(k, _)| k.clone())
                .collect()
        })
        .unwrap_or_default()
}

fn ref_name(value: &Value) -> Option<String> {
    value
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|r| r.strip_prefix("#/components/schemas/"))
        .map(|n| n.replace("~1", "/").replace("~0", "~"))
}

/// Collects every definition of property `field` reachable through $ref and
/// allOf/anyOf/oneOf, with whether it is required at the node that declares it.
fn find_property(
    schemas: &Map<String, Value>,
    schema: &Value,
    field: &str,
    found: &mut Vec<(Value, bool)>,
    visiting: &mut BTreeSet<String>,
) {
    if let Some(name) = ref_name(schema) {
        if visiting.insert(name.clone()) {
            if let Some(target) = schemas.get(&name) {
                find_property(schemas, target, field, found, visiting);
            }
            visiting.remove(&name);
        }
        return;
    }
    if let Some(property) = schema.get("properties").and_then(|p| p.get(field)) {
        let required = schema
            .get("required")
            .and_then(Value::as_array)
            .is_some_and(|r| r.iter().any(|v| v == field));
        found.push((property.clone(), required));
    }
    for key in ["allOf", "anyOf", "oneOf"] {
        if let Some(variants) = schema.get(key).and_then(Value::as_array) {
            for variant in variants {
                find_property(schemas, variant, field, found, visiting);
            }
        }
    }
}

const K8S_KEEP: &[&str] = &[
    "description",
    "format",
    "minimum",
    "maximum",
    "minLength",
    "maxLength",
    "pattern",
    "minItems",
    "maxItems",
    "title",
];

/// Converts a (3.1-normalized) OpenAPI schema into a Kubernetes structural schema:
/// inline $ref, flatten compositions, `type: [T, null]` -> nullable, drop
/// keywords apiextensions does not accept.
fn k8sify(schemas: &Map<String, Value>, schema: &Value, stack: &mut Vec<String>) -> Value {
    if let Some(name) = ref_name(schema) {
        if stack.contains(&name) {
            return serde_json::json!({"x-kubernetes-preserve-unknown-fields": true});
        }
        stack.push(name.clone());
        let mut merged = schemas
            .get(&name)
            .map_or(serde_json::json!({}), |t| k8sify(schemas, t, stack));
        stack.pop();
        // sibling keywords next to $ref (e.g. description) win
        if let (Some(m), Some(desc)) = (merged.as_object_mut(), schema.get("description")) {
            m.insert("description".into(), desc.clone());
        }
        return merged;
    }

    let mut out = Map::new();
    let mut nullable = false;
    let mut ty: Option<String> = match schema.get("type") {
        Some(Value::String(t)) => Some(t.clone()),
        Some(Value::Array(types)) => {
            let non_null: Vec<&str> = types
                .iter()
                .filter_map(Value::as_str)
                .filter(|t| *t != "null")
                .collect();
            nullable = types.iter().any(|t| t == "null");
            (non_null.len() == 1).then(|| non_null[0].to_owned())
        }
        _ => None,
    };
    for key in K8S_KEEP {
        if let Some(v) = schema.get(*key) {
            out.insert((*key).to_owned(), v.clone());
        }
    }
    if let Some(values) = schema.get("enum").and_then(Value::as_array) {
        nullable |= values.iter().any(Value::is_null);
        let values: Vec<Value> = values.iter().filter(|v| !v.is_null()).cloned().collect();
        if !values.is_empty() {
            out.insert("enum".into(), Value::Array(values));
        }
    }

    let mut properties = Map::new();
    let mut required: BTreeSet<String> = BTreeSet::new();
    if let Some(props) = schema.get("properties").and_then(Value::as_object) {
        for (name, prop) in props {
            properties.insert(name.clone(), k8sify(schemas, prop, stack));
        }
        if let Some(r) = schema.get("required").and_then(Value::as_array) {
            required.extend(r.iter().filter_map(Value::as_str).map(str::to_owned));
        }
    }

    // Flatten compositions into the node itself.
    let mut variant_types = BTreeSet::new();
    let mut enums: Vec<Value> = Vec::new();
    let mut enum_complete = true;
    for key in ["allOf", "anyOf", "oneOf"] {
        let Some(variants) = schema.get(key).and_then(Value::as_array) else {
            continue;
        };
        for variant in variants {
            let v = k8sify(schemas, variant, stack);
            if let Some(t) = v.get("type").and_then(Value::as_str) {
                variant_types.insert(t.to_owned());
            }
            if v.get("nullable") == Some(&Value::Bool(true)) {
                nullable = true;
            }
            match v.get("enum").and_then(Value::as_array) {
                Some(e) => enums.extend(e.iter().cloned()),
                None => enum_complete = false,
            }
            if let Some(p) = v.get("properties").and_then(Value::as_object) {
                for (name, prop) in p {
                    properties
                        .entry(name.clone())
                        .or_insert_with(|| prop.clone());
                }
            }
            if key == "allOf" {
                if let Some(r) = v.get("required").and_then(Value::as_array) {
                    required.extend(r.iter().filter_map(Value::as_str).map(str::to_owned));
                }
            }
            for k in K8S_KEEP {
                if let (Some(val), false) = (v.get(*k), out.contains_key(*k)) {
                    if *k == "description" || *k == "title" {
                        out.insert((*k).to_owned(), val.clone());
                    }
                }
            }
            if let Some(items) = v.get("items") {
                out.entry("items").or_insert_with(|| items.clone());
            }
            if v.get("x-kubernetes-preserve-unknown-fields").is_some() {
                variant_types.insert("any".into());
            }
        }
        if enum_complete && !enums.is_empty() && !out.contains_key("enum") {
            let mut unique = Vec::new();
            for e in enums.drain(..) {
                if !unique.contains(&e) {
                    unique.push(e);
                }
            }
            out.insert("enum".into(), Value::Array(unique));
        }
    }
    if ty.is_none() {
        match variant_types.len() {
            1 if !variant_types.contains("any") => ty = variant_types.into_iter().next(),
            0 if !properties.is_empty() => ty = Some("object".into()),
            0 => {}
            _ => {
                let int_or_string = variant_types
                    .iter()
                    .all(|t| matches!(t.as_str(), "integer" | "string" | "number"));
                out.retain(|k, _| k == "description" || k == "title");
                if int_or_string && variant_types.contains("string") {
                    out.insert("x-kubernetes-int-or-string".into(), Value::Bool(true));
                } else {
                    out.insert(
                        "x-kubernetes-preserve-unknown-fields".into(),
                        Value::Bool(true),
                    );
                }
                if nullable {
                    out.insert("nullable".into(), Value::Bool(true));
                }
                return Value::Object(out);
            }
        }
    }

    match ty.as_deref() {
        Some("object") => {
            out.insert("type".into(), "object".into());
            if !properties.is_empty() {
                let required: Vec<Value> = required
                    .into_iter()
                    .filter(|r| properties.contains_key(r))
                    .map(Value::String)
                    .collect();
                out.insert("properties".into(), Value::Object(properties));
                if !required.is_empty() {
                    out.insert("required".into(), Value::Array(required));
                }
            } else {
                match schema.get("additionalProperties") {
                    Some(ap) if ap.is_object() => {
                        out.insert("additionalProperties".into(), k8sify(schemas, ap, stack));
                    }
                    _ => {
                        out.insert(
                            "x-kubernetes-preserve-unknown-fields".into(),
                            Value::Bool(true),
                        );
                    }
                }
            }
        }
        Some("array") => {
            out.insert("type".into(), "array".into());
            let items = schema.get("items").map_or_else(
                || {
                    out.get("items").cloned().unwrap_or(
                        serde_json::json!({"x-kubernetes-preserve-unknown-fields": true}),
                    )
                },
                |i| k8sify(schemas, i, stack),
            );
            out.insert("items".into(), items);
        }
        Some(t) => {
            out.insert("type".into(), Value::String(t.to_owned()));
        }
        None => {
            out.retain(|k, _| k == "description" || k == "title");
            out.insert(
                "x-kubernetes-preserve-unknown-fields".into(),
                Value::Bool(true),
            );
        }
    }
    if nullable {
        out.insert("nullable".into(), Value::Bool(true));
    }
    Value::Object(out)
}

fn drop_default_only_all_of(value: &mut Value) -> usize {
    let mut count = 0;
    match value {
        Value::Object(map) => {
            if let Some(Value::Array(items)) = map.get_mut("allOf") {
                let before = items.len();
                items.retain(|item| {
                    !(item
                        .as_object()
                        .is_some_and(|o| o.len() == 1 && o.contains_key("default")))
                });
                count += before - items.len();
            }
            for v in map.values_mut() {
                count += drop_default_only_all_of(v);
            }
        }
        Value::Array(items) => {
            for v in items {
                count += drop_default_only_all_of(v);
            }
        }
        _ => {}
    }
    count
}

/// Compares roas' 3.1 output with our own minimal normalization, schema by schema,
/// listing keys that roas dropped or changed.
fn report_roundtrip_diff(own: &Value, roas: &Value) {
    let (Some(a), Some(b)) = (
        own.pointer("/components/schemas")
            .and_then(Value::as_object),
        roas.pointer("/components/schemas")
            .and_then(Value::as_object),
    ) else {
        return;
    };
    let mut diffs = Vec::new();
    for (name, schema) in a {
        let other = b.get(name).unwrap_or(&Value::Null);
        diff_values(&format!("{name}"), schema, other, &mut diffs);
    }
    let mut kinds: std::collections::BTreeMap<String, usize> = Default::default();
    for d in &diffs {
        let key = d.split(": ").nth(1).unwrap_or("").to_owned();
        *kinds.entry(key).or_default() += 1;
    }
    println!(
        "[roas vs own normalizer] {} difference(s) in components.schemas",
        diffs.len()
    );
    for (kind, n) in &kinds {
        println!("    {n:>4} x {kind}");
    }
    for d in diffs
        .iter()
        .filter(|d| {
            [
                "`enum`",
                "`required`",
                "`minLength`",
                "`description`",
                "`default`",
            ]
            .iter()
            .any(|k| d.contains(k))
        })
        .take(12)
    {
        println!("    e.g. {d}");
    }
}

fn diff_values(path: &str, a: &Value, b: &Value, out: &mut Vec<String>) {
    match (a, b) {
        (Value::Object(x), Value::Object(y)) => {
            for (k, v) in x {
                match y.get(k) {
                    Some(w) => diff_values(&format!("{path}.{k}"), v, w, out),
                    None => out.push(format!("{path}.{k}: dropped `{k}`")),
                }
            }
            for k in y.keys().filter(|k| !x.contains_key(*k)) {
                out.push(format!("{path}.{k}: added `{k}`"));
            }
        }
        (Value::Array(x), Value::Array(y)) if x.len() == y.len() => {
            for (i, (v, w)) in x.iter().zip(y).enumerate() {
                diff_values(&format!("{path}[{i}]"), v, w, out);
            }
        }
        (x, y) if x != y => {
            let key = path.rsplit('.').next().unwrap_or(path);
            out.push(format!("{path}: changed `{key}`"));
        }
        _ => {}
    }
}

/// Read-only typed access through openapiv3: can it parse the full document, and
/// do x-kube extensions stay reachable?
fn check_openapiv3(doc: &Value) {
    let t = Instant::now();
    match serde_path_to_error::deserialize::<_, openapiv3::OpenAPI>(doc.clone()) {
        Err(e) => println!("[openapiv3 full parse] FAILED in {:?}: {e}", t.elapsed()),
        Ok(api) => {
            let elapsed = t.elapsed();
            let ops: Vec<String> = api
                .operations()
                .filter(|(_, _, op)| op.extensions.contains_key("x-kube-operation"))
                .map(|(path, method, _)| format!("{} {path}", method.to_uppercase()))
                .collect();
            let resources = api.components.as_ref().map_or(0, |c| {
                c.schemas
                    .values()
                    .filter(|s| match s {
                        openapiv3::ReferenceOr::Item(item) => {
                            item.schema_data.extensions.contains_key("x-kube-resource")
                        }
                        openapiv3::ReferenceOr::Reference { .. } => false,
                    })
                    .count()
            });
            println!(
                "[openapiv3 full parse] ok in {elapsed:?}: {} paths, {} x-kube operations, {resources} x-kube resources",
                api.paths.paths.len(),
                ops.len()
            );
        }
    }
}

/// Flattens anyOf/oneOf (through $ref) into leaf variants, keeping the name of the
/// component each leaf came from.
fn collect_variants(
    schemas: &Map<String, Value>,
    schema: &Value,
    name: Option<&str>,
    out: &mut Vec<(String, Value)>,
    visiting: &mut BTreeSet<String>,
) {
    if let Some(target) = ref_name(schema) {
        if visiting.insert(target.clone()) {
            if let Some(resolved) = schemas.get(&target) {
                collect_variants(schemas, resolved, Some(&target), out, visiting);
            }
            visiting.remove(&target);
        }
        return;
    }
    let composition = schema
        .get("oneOf")
        .or_else(|| schema.get("anyOf"))
        .and_then(Value::as_array);
    match composition {
        Some(items) => items
            .iter()
            .for_each(|item| collect_variants(schemas, item, None, out, visiting)),
        None => out.push((name.unwrap_or("Variant").to_owned(), schema.clone())),
    }
}

/// Object with only the listed fields found in `schema` (through $ref/allOf/...).
fn filter_fields(schemas: &Map<String, Value>, schema: &Value, fields: &[&str]) -> Value {
    let mut properties = Map::new();
    let mut required = Vec::new();
    for field in fields {
        let mut found = Vec::new();
        find_property(schemas, schema, field, &mut found, &mut BTreeSet::new());
        let mut distinct: Vec<(Value, bool)> = Vec::new();
        for (s, r) in found {
            if !distinct.iter().any(|(d, _)| d == &s) {
                distinct.push((s, r));
            }
        }
        match distinct.len() {
            0 => {}
            1 => {
                let (s, r) = distinct.remove(0);
                if r {
                    required.push(Value::String((*field).to_owned()));
                }
                properties.insert((*field).to_owned(), s);
            }
            _ => {
                if distinct.iter().all(|(_, r)| *r) {
                    required.push(Value::String((*field).to_owned()));
                }
                properties.insert(
                    (*field).to_owned(),
                    serde_json::json!({
                        "anyOf": distinct.into_iter().map(|(s, _)| s).collect::<Vec<_>>()
                    }),
                );
            }
        }
    }
    serde_json::json!({"type": "object", "properties": properties, "required": required})
}
