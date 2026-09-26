use std::collections::BTreeMap;

use crate::ir::{
    ApiKeyLocationIr, CredentialsIr, FieldSelectionIr, HttpMethod, LifecycleIr, OperationId,
    OperationIr, ProviderIr, ReferenceIr, ResourceIr, ResourceName, SchemaName, SecuritySchemeIr,
    SecuritySchemeName, ValueExpr,
};
use crate::loader::LoaderError;
use openapiv3::{
    APIKeyLocation, Content, OpenAPI, Operation, ReferenceOr, SecurityScheme, StatusCode,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResourceExtension {
    name: ResourceName,
    api_version: String,
    kind: String,
    #[serde(default)]
    fields: FieldSelectionIr,
    #[serde(default)]
    identifiers: BTreeMap<String, ValueExpr>,
    #[serde(default)]
    references: BTreeMap<String, ReferenceIr>,
    credentials: Option<CredentialsIr>,
}

#[derive(Debug, Deserialize)]
struct OperationExtension {
    resource: ResourceName,
    lifecycle: LifecycleAction,
    #[serde(default)]
    path_parameters: BTreeMap<String, ValueExpr>,
    #[serde(default)]
    response: Option<ResponseExtension>,
}

#[derive(Debug, Deserialize)]
struct ResponseExtension {
    path: String,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum LifecycleAction {
    Observe,
    Create,
    Update,
    Delete,
}

impl LifecycleAction {
    fn name(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Create => "create",
            Self::Update => "update",
            Self::Delete => "delete",
        }
    }
}

#[derive(Debug, Default)]
struct LifecycleDraft {
    observe: Option<OperationId>,
    create: Option<OperationId>,
    update: Option<OperationId>,
    delete: Option<OperationId>,
}

/// Builds a [`ProviderIr`] from a document prepared by [`crate::spec::prepare`]
/// (pruned, with operation bodies as component references).
///
/// # Errors
///
/// Returns [`LoaderError`] when the document is not valid `OpenAPI` 3.0, an
/// `x-kube-*` extension is malformed, a resource has no observe operation or
/// duplicate lifecycle operations, or a security scheme is unsupported.
pub fn extract(prepared: &Value) -> Result<ProviderIr, LoaderError> {
    let api: OpenAPI = serde_json::from_value(prepared.clone())
        .map_err(|error| LoaderError::InvalidOpenApi(error.to_string()))?;

    let mut resources = extract_resources(&api)?;
    let mut lifecycles: BTreeMap<ResourceName, LifecycleDraft> = BTreeMap::new();
    let mut operations = BTreeMap::new();

    for (path, method, operation) in api.operations() {
        let Some(extension) = operation.extensions.get("x-kube-operation") else {
            continue;
        };
        let location = format!("paths.{path}.{method}");
        let extension: OperationExtension =
            serde_json::from_value(extension.clone()).map_err(|error| {
                ir_error(format!("invalid x-kube-operation at {location}: {error}"))
            })?;

        if !resources.contains_key(&extension.resource) {
            return Err(ir_error(format!(
                "{location} references unknown resource {}",
                extension.resource.0
            )));
        }

        let id = OperationId(format!(
            "{}.{}",
            extension.resource.0,
            extension.lifecycle.name()
        ));
        let lifecycle = lifecycles.entry(extension.resource.clone()).or_default();
        let slot = match extension.lifecycle {
            LifecycleAction::Observe => &mut lifecycle.observe,
            LifecycleAction::Create => &mut lifecycle.create,
            LifecycleAction::Update => &mut lifecycle.update,
            LifecycleAction::Delete => &mut lifecycle.delete,
        };
        if slot.replace(id.clone()).is_some() {
            return Err(ir_error(format!(
                "resource {} has duplicate {} operation",
                extension.resource.0,
                extension.lifecycle.name()
            )));
        }

        operations.insert(
            id,
            OperationIr {
                method: parse_method(method)?,
                path: path.to_owned(),
                path_parameters: extension.path_parameters,
                request: request_schema(operation),
                response: response_schema(operation),
                response_path: extension.response.map(|response| response.path),
            },
        );
    }

    for (name, resource) in &mut resources {
        let draft = lifecycles.remove(name).unwrap_or_default();
        resource.lifecycle = LifecycleIr {
            observe: draft
                .observe
                .ok_or_else(|| ir_error(format!("resource {} has no observe operation", name.0)))?,
            create: draft.create,
            update: draft.update,
            delete: draft.delete,
        };
    }

    let provider = api
        .extensions
        .get("x-kube-provider")
        .and_then(|provider| provider.get("name"))
        .and_then(Value::as_str)
        .unwrap_or(&api.info.title)
        .to_owned();

    let base_url = api
        .servers
        .first()
        .map(|server| server.url.clone())
        .unwrap_or_default();

    Ok(ProviderIr {
        version: 1,
        provider,
        base_url,
        resources,
        operations,
        security_schemes: extract_security_schemes(&api),
    })
}

fn extract_resources(api: &OpenAPI) -> Result<BTreeMap<ResourceName, ResourceIr>, LoaderError> {
    let mut resources = BTreeMap::new();
    let Some(components) = &api.components else {
        return Ok(resources);
    };

    for (schema_name, schema) in &components.schemas {
        let ReferenceOr::Item(schema) = schema else {
            continue;
        };
        let Some(extension) = schema.schema_data.extensions.get("x-kube-resource") else {
            continue;
        };
        let extension: ResourceExtension =
            serde_json::from_value(extension.clone()).map_err(|error| {
                ir_error(format!(
                    "invalid x-kube-resource on schema {schema_name}: {error}"
                ))
            })?;

        let name = extension.name.clone();
        let resource = ResourceIr {
            api_version: extension.api_version,
            kind: extension.kind,
            identifiers: extension.identifiers,
            field_selection: extension.fields,
            references: extension.references,
            credentials: extension.credentials,
            lifecycle: LifecycleIr {
                observe: OperationId(String::new()),
                create: None,
                update: None,
                delete: None,
            },
        };
        if resources.insert(name.clone(), resource).is_some() {
            return Err(ir_error(format!("duplicate resource {}", name.0)));
        }
    }
    Ok(resources)
}

fn request_schema(operation: &Operation) -> Option<SchemaName> {
    match operation.request_body.as_ref()? {
        ReferenceOr::Item(body) => json_schema_name(&body.content),
        ReferenceOr::Reference { .. } => None,
    }
}

fn response_schema(operation: &Operation) -> Option<SchemaName> {
    let mut successful: Vec<(u16, &ReferenceOr<openapiv3::Response>)> = operation
        .responses
        .responses
        .iter()
        .filter_map(|(status, response)| match status {
            StatusCode::Code(code) if (200..300).contains(code) => Some((*code, response)),
            StatusCode::Range(2) => Some((200, response)),
            _ => None,
        })
        .collect();
    successful.sort_by_key(|(code, _)| *code);
    successful
        .into_iter()
        .find_map(|(_, response)| match response {
            ReferenceOr::Item(response) => json_schema_name(&response.content),
            ReferenceOr::Reference { .. } => None,
        })
}

/// Component name of the JSON media type's schema, if it is a `$ref`.
fn json_schema_name(content: &Content) -> Option<SchemaName> {
    let media = content.get("application/json").or_else(|| {
        content
            .iter()
            .find(|(name, _)| name.ends_with("+json"))
            .map(|(_, media)| media)
    })?;
    match media.schema.as_ref()? {
        ReferenceOr::Reference { reference } => reference
            .strip_prefix("#/components/schemas/")
            .map(|name| SchemaName(name.replace("~1", "/").replace("~0", "~"))),
        ReferenceOr::Item(_) => None,
    }
}

fn extract_security_schemes(api: &OpenAPI) -> BTreeMap<SecuritySchemeName, SecuritySchemeIr> {
    let mut result = BTreeMap::new();
    let Some(components) = &api.components else {
        return result;
    };
    for (name, scheme) in &components.security_schemes {
        let ReferenceOr::Item(scheme) = scheme else {
            continue;
        };
        let scheme = match scheme {
            SecurityScheme::APIKey { location, name, .. } => SecuritySchemeIr::ApiKey {
                name: name.clone(),
                location: match location {
                    APIKeyLocation::Header => ApiKeyLocationIr::Header,
                    APIKeyLocation::Query => ApiKeyLocationIr::Query,
                    APIKeyLocation::Cookie => ApiKeyLocationIr::Cookie,
                },
            },
            SecurityScheme::HTTP {
                scheme,
                bearer_format,
                ..
            } => SecuritySchemeIr::Http {
                scheme: scheme.to_ascii_lowercase(),
                bearer_format: bearer_format.clone(),
            },
            SecurityScheme::OAuth2 { .. } | SecurityScheme::OpenIDConnect { .. } => {
                // Not usable for operator credentials; resources cannot select them
                // (validation reports an unknown scheme).
                continue;
            }
        };
        result.insert(SecuritySchemeName(name.clone()), scheme);
    }
    result
}

fn parse_method(method: &str) -> Result<HttpMethod, LoaderError> {
    match method {
        "get" => Ok(HttpMethod::GET),
        "post" => Ok(HttpMethod::POST),
        "put" => Ok(HttpMethod::PUT),
        "patch" => Ok(HttpMethod::PATCH),
        "delete" => Ok(HttpMethod::DELETE),
        _ => Err(ir_error(format!("unsupported HTTP method {method}"))),
    }
}

fn ir_error(message: impl Into<String>) -> LoaderError {
    LoaderError::Ir(message.into())
}
