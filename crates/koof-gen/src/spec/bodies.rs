use super::{COMPONENT_SCHEMA_PREFIX, SpecError, json_media_mut, schemas_mut};
use heck::ToUpperCamelCase;
use serde::Deserialize;
use serde_json::{Map, Value, json};

#[derive(Debug, Deserialize)]
struct OperationExtension {
    resource: String,
    lifecycle: String,
}

/// Makes every JSON request body and first successful JSON response of an
/// `x-kube-operation` a `components.schemas` reference, so each has a type name.
/// Inline schemas move to `<Resource><Lifecycle>Request` / `...Response`; bodies
/// referenced from `components.requestBodies` / `components.responses` are inlined
/// into the operation first.
///
/// # Errors
///
/// Returns [`SpecError`] when an `x-kube-operation` is malformed or a new schema name
/// collides with an existing one.
pub fn hoist_operation_bodies(document: &mut Value) -> Result<(), SpecError> {
    let snapshot = document.clone();
    let mut hoisted = Map::new();

    let Some(paths) = document.get_mut("paths").and_then(Value::as_object_mut) else {
        return Ok(());
    };
    for (path, item) in paths.iter_mut() {
        let Some(item) = item.as_object_mut() else {
            continue;
        };
        for (method, operation) in item.iter_mut() {
            let Some(extension) = operation.get("x-kube-operation") else {
                continue;
            };
            let extension: OperationExtension =
                serde_json::from_value(extension.clone()).map_err(|source| {
                    SpecError::Extension {
                        extension: "x-kube-operation",
                        location: format!("paths.{path}.{method}"),
                        source,
                    }
                })?;
            let base = format!(
                "{}{}",
                extension.resource.to_upper_camel_case(),
                extension.lifecycle.to_upper_camel_case()
            );

            if let Some(body) = operation.get_mut("requestBody") {
                inline_reference(&snapshot, body);
                hoist(body, &format!("{base}Request"), &mut hoisted);
            }

            if let Some(responses) = operation
                .get_mut("responses")
                .and_then(Value::as_object_mut)
            {
                let mut statuses: Vec<String> = responses
                    .keys()
                    .filter(|status| status.starts_with('2'))
                    .cloned()
                    .collect();
                statuses.sort();
                for status in statuses {
                    let Some(response) = responses.get_mut(&status) else {
                        continue;
                    };
                    inline_reference(&snapshot, response);
                    if hoist(response, &format!("{base}Response"), &mut hoisted) {
                        break;
                    }
                }
            }
        }
    }

    let schemas = schemas_mut(document)?;
    for (name, schema) in hoisted {
        if schemas.contains_key(&name) {
            return Err(SpecError::Resource {
                resource: name.clone(),
                message: format!("hoisted body schema {name} collides with an existing schema"),
            });
        }
        schemas.insert(name, schema);
    }
    Ok(())
}

/// Replaces a `{"$ref": "#/components/..."}` request body / response with a copy of
/// its target, so it can be changed for this operation only.
fn inline_reference(document: &Value, value: &mut Value) {
    if let Some(target) = value
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|reference| reference.strip_prefix('#'))
        .and_then(|pointer| document.pointer(pointer))
    {
        *value = target.clone();
    }
}

/// Moves an inline JSON schema of `container` (a request body or response) to
/// `name`; returns whether the container has a JSON schema at all.
fn hoist(container: &mut Value, name: &str, hoisted: &mut Map<String, Value>) -> bool {
    let Some(media) = container
        .get_mut("content")
        .and_then(Value::as_object_mut)
        .and_then(json_media_mut)
    else {
        return false;
    };
    let Some(schema) = media.get_mut("schema") else {
        return false;
    };
    let is_component_ref = schema
        .get("$ref")
        .and_then(Value::as_str)
        .is_some_and(|reference| reference.starts_with(COMPONENT_SCHEMA_PREFIX));
    if !is_component_ref {
        let inline = std::mem::replace(
            schema,
            json!({ "$ref": format!("{COMPONENT_SCHEMA_PREFIX}{name}") }),
        );
        hoisted.insert(name.to_owned(), inline);
    }
    true
}
