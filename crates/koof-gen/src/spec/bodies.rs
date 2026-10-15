use super::{
    COMPONENT_SCHEMA_PREFIX, SpecError, extensions::kube_operations, json_media_mut, schemas_mut,
    success_statuses,
};
use heck::ToUpperCamelCase;
use serde_json::{Map, Value, json};

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

    for kube_operation in kube_operations(&snapshot) {
        let kube_operation = kube_operation?;
        let extension = &kube_operation.extension;
        let base = format!(
            "{}{}",
            extension.resource.0.to_upper_camel_case(),
            extension.lifecycle.name().to_upper_camel_case()
        );
        let Some(operation) = document
            .get_mut("paths")
            .and_then(|paths| paths.get_mut(kube_operation.path))
            .and_then(|item| item.get_mut(kube_operation.method))
        else {
            continue;
        };

        if let Some(body) = operation.get_mut("requestBody") {
            inline_reference(&snapshot, body);
            hoist(body, &format!("{base}Request"), &mut hoisted);
        }

        if let Some(responses) = operation
            .get_mut("responses")
            .and_then(Value::as_object_mut)
        {
            for status in success_statuses(responses) {
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
