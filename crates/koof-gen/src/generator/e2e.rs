//! [Chainsaw](https://kyverno.github.io/chainsaw) end-to-end tests
use super::{GeneratedFile, GeneratorError};
use crate::ir::resolved::{Provider, Resource};
use serde_json::{Map, Value, json};

const SECRET: &str = "credentials";
const SECRET_KEY: &str = "token";
const HEADER: &str =
    "# This file is generated from the x-kube-resource test fixtures. Do not edit manually.\n";

/// `e2e/generated/<resource>/chainsaw-test.yaml` for every resource with a fixture.
///
/// # Errors
///
/// Returns [`GeneratorError::Yaml`] when a test cannot be serialized.
pub fn generate_tests(provider: &Provider<'_>) -> Result<Vec<GeneratedFile>, GeneratorError> {
    provider
        .resources
        .iter()
        .filter(|resource| resource.test.is_some())
        .map(|resource| {
            let name = resource.name.0.replace('_', "-");
            Ok(GeneratedFile {
                path: format!("e2e/generated/{name}/chainsaw-test.yaml").into(),
                content: format!(
                    "{HEADER}{}",
                    serde_yaml::to_string(&test(provider, resource, &name))?
                ),
            })
        })
        .collect()
}

fn test(provider: &Provider<'_>, resource: &Resource<'_>, name: &str) -> Value {
    let objects = dependency_order(provider, resource);

    let mut create: Vec<Value> = Vec::new();
    if objects.iter().any(|object| {
        object
            .credentials
            .as_ref()
            .is_some_and(|c| c.secret_field.is_some())
    }) {
        create.push(json!({ "apply": { "resource": {
            "apiVersion": "v1",
            "kind": "Secret",
            "metadata": { "name": SECRET },
            "stringData": { SECRET_KEY: "fake-token" },
        } } }));
    }
    create.extend(
        objects
            .iter()
            .map(|object| json!({ "apply": { "resource": manifest(provider, object) } })),
    );
    create.extend(objects.iter().map(
        |object| json!({ "assert": { "resource": with_status(object, condition("Ready")) } }),
    ));

    let mut steps = vec![json!({ "name": "create", "try": create })];
    if let (Some(_), Some(update)) = (
        resource.update,
        resource
            .test
            .and_then(|test| test.update.as_ref())
            .map(|update| literal(&Value::Object(update.clone()))),
    ) {
        let mut patch = reference(resource);
        patch["spec"] = json!({ "forProvider": update });
        let mut status = condition("Synced");
        status.insert("atProvider".to_owned(), update);
        steps.push(json!({ "name": "update", "try": [
            { "patch": { "resource": patch } },
            { "assert": { "resource": with_status(resource, status) } },
        ] }));
    }
    if resource.delete.is_some() {
        steps.push(json!({ "name": "delete", "try": [{ "delete": { "ref": {
            "apiVersion": api_version(resource),
            "kind": resource.kind,
            "name": object_name(resource),
        } } }] }));
    }

    json!({
        "apiVersion": "chainsaw.kyverno.io/v1alpha1",
        "kind": "Test",
        "metadata": { "name": name },
        "spec": { "steps": steps },
    })
}

fn dependency_order<'p, 'a>(
    provider: &'p Provider<'a>,
    resource: &'p Resource<'a>,
) -> Vec<&'p Resource<'a>> {
    fn visit<'p, 'a>(
        provider: &'p Provider<'a>,
        resource: &'p Resource<'a>,
        order: &mut Vec<&'p Resource<'a>>,
    ) {
        if order.iter().any(|seen| seen.name == resource.name) {
            return;
        }
        for reference in &resource.references {
            if let Some(target) = find(provider, reference.target) {
                visit(provider, target, order);
            }
        }
        order.push(resource);
    }
    let mut order = Vec::new();
    visit(provider, resource, &mut order);
    order
}

fn find<'p, 'a>(
    provider: &'p Provider<'a>,
    name: &crate::ir::ResourceName,
) -> Option<&'p Resource<'a>> {
    provider
        .resources
        .iter()
        .find(|resource| resource.name == name)
}

fn api_version(resource: &Resource<'_>) -> String {
    format!("{}/{}", resource.group, resource.version)
}

fn object_name(resource: &Resource<'_>) -> String {
    if resource.namespaced {
        resource.name.0.replace('_', "-")
    } else {
        "($namespace)".to_owned()
    }
}

fn reference(resource: &Resource<'_>) -> Value {
    json!({
        "apiVersion": api_version(resource),
        "kind": resource.kind,
        "metadata": { "name": object_name(resource) },
    })
}

fn manifest(provider: &Provider<'_>, resource: &Resource<'_>) -> Value {
    let mut for_provider = resource
        .test
        .and_then(|test| {
            literal(&Value::Object(test.for_provider.clone()))
                .as_object()
                .cloned()
        })
        .unwrap_or_default();
    for reference in &resource.references {
        if let Some(target) = find(provider, reference.target) {
            for_provider.insert(
                reference.field.to_owned(),
                json!({ "name": object_name(target) }),
            );
        }
    }
    if let Some(field) = resource
        .credentials
        .as_ref()
        .and_then(|credentials| credentials.secret_field)
    {
        for_provider.insert(
            field.to_owned(),
            json!({ "name": SECRET, "key": SECRET_KEY, "namespace": "($namespace)" }),
        );
    }

    let mut object = reference(resource);
    if resource.create.is_none() {
        object["metadata"]["annotations"] = json!({
            format!("{}/external-name", resource.group): "($namespace)",
        });
    }
    let mut spec = Map::new();
    spec.insert("forProvider".to_owned(), Value::Object(for_provider));
    if resource.delete.is_none() {
        spec.insert("deletionPolicy".to_owned(), json!("Orphan"));
    }
    object["spec"] = Value::Object(spec);
    object
}

fn literal(value: &Value) -> Value {
    match value {
        Value::String(text) if text.starts_with('(') && text.ends_with(')') => Value::String(
            format!("('{}')", text.replace('\\', "\\\\").replace('\'', "\\'")),
        ),
        Value::Array(items) => Value::Array(items.iter().map(literal).collect()),
        Value::Object(object) => Value::Object(
            object
                .iter()
                .map(|(key, value)| (key.clone(), literal(value)))
                .collect(),
        ),
        other => other.clone(),
    }
}

fn condition(type_: &str) -> Map<String, Value> {
    let mut status = Map::new();
    status.insert(
        format!("(conditions[?type == '{type_}'])"),
        json!([{ "status": "True" }]),
    );
    status
}

fn with_status(resource: &Resource<'_>, status: Map<String, Value>) -> Value {
    let mut object = reference(resource);
    object["status"] = Value::Object(status);
    object
}
