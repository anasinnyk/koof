use super::{
    ApiKeyLocationIr, HttpIr, HttpMethod, LifecycleIr, OperationId, OperationIr, ProviderIr, ResourceIr,
    ResourceName, SchemaName, SecuritySchemeIr, SecuritySchemeName,
};
use crate::spec::{
    self, SpecError,
    extensions::{
        HttpExtension, LifecycleAction, OperationExtension, ResourceExtension, kube_operations,
        kube_provider,
        kube_resources,
    },
};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Debug, thiserror::Error)]
pub enum ExtractError {
    #[error(transparent)]
    Extension(#[from] SpecError),

    #[error("failed to build ProviderIr: {0}")]
    Ir(String),

    #[error("invalid OpenAPI: {0}")]
    InvalidOpenApi(String),
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
/// Returns [`ExtractError`] when the document has no title, an `x-kube-*`
/// extension is malformed, a resource has no observe operation or duplicate
/// lifecycle operations, or an operation uses an unsupported HTTP method.
pub fn extract(prepared: &Value) -> Result<ProviderIr, ExtractError> {
    let resources = extract_resources(prepared)?;
    let mut lifecycles: BTreeMap<ResourceName, LifecycleDraft> = BTreeMap::new();
    let mut operations = BTreeMap::new();

    for kube_operation in kube_operations(prepared) {
        let kube_operation = kube_operation?;
        let location = kube_operation.location();
        let extension = kube_operation.extension;

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
        let (response_path, deleted_when) = response_fields(&extension)?;

        operations.insert(
            id,
            OperationIr {
                method: parse_method(kube_operation.method)?,
                path: kube_operation.path.to_owned(),
                path_parameters: extension.path_parameters,
                request: spec::request_schema_name(kube_operation.operation).map(SchemaName),
                response: spec::response_schema_name(prepared, kube_operation.operation)
                    .map(SchemaName),
                response_path,
                deleted_when,
                request_mapping: extension.request,
                result_identifiers: extension.result.unwrap_or_default().identifiers,
            },
        );
    }

    let resources = resources
        .into_iter()
        .map(|(name, extension)| {
            let lifecycle = lifecycles.remove(&name).unwrap_or_default();
            let observe = lifecycle
                .observe
                .ok_or_else(|| ir_error(format!("resource {} has no observe operation", name.0)))?;
            let resource = ResourceIr {
                api_version: extension.api_version,
                kind: extension.kind,
                cluster: extension.cluster,
                identifiers: extension.identifiers,
                references: extension.references,
                credentials: extension.credentials,
                validations: extension.validations,
                immutable: extension.immutable,
                test: extension.test,
                lifecycle: LifecycleIr {
                    observe,
                    create: lifecycle.create,
                    update: lifecycle.update,
                    delete: lifecycle.delete,
                },
            };
            Ok((name, resource))
        })
        .collect::<Result<_, ExtractError>>()?;

    let extension = kube_provider(prepared)?;
    let provider = match extension.as_ref().map(|extension| extension.name.as_str()).or_else(|| {
        prepared.pointer("/info/title").and_then(Value::as_str)
    }) {
        Some(provider) => provider.to_owned(),
        None => return Err(ExtractError::InvalidOpenApi("missing info.title".to_owned())),
    };
    let http = http_defaults(extension.map(|extension| extension.http).unwrap_or_default())?;

    let base_url = prepared
        .pointer("/servers/0/url")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();

    Ok(ProviderIr {
        provider,
        base_url,
        http,
        resources,
        operations,
        security_schemes: extract_security_schemes(prepared)?,
    })
}

fn http_defaults(extension: HttpExtension) -> Result<HttpIr, ExtractError> {
    let duration = |name: &str, value: Option<String>| {
        value
            .map(|value| {
                humantime::parse_duration(&value).map_err(|error| {
                    ir_error(format!(
                        "x-kube-provider.http.{name}: {value:?} is not a duration like 30s: {error}"
                    ))
                })
            })
            .transpose()
    };
    Ok(HttpIr {
        timeout: duration("timeout", extension.timeout)?,
        connect_timeout: duration("connect_timeout", extension.connect_timeout)?,
        pool_idle_timeout: duration("pool_idle_timeout", extension.pool_idle_timeout)?,
        pool_max_idle_per_host: extension.pool_max_idle_per_host,
        user_agent: extension.user_agent,
    })
}

fn extract_resources(
    document: &Value,
) -> Result<BTreeMap<ResourceName, ResourceExtension>, ExtractError> {
    let mut resources = BTreeMap::new();
    for kube_resource in kube_resources(document) {
        let extension = kube_resource?.extension;
        let name = extension.name.clone();
        if resources.insert(name.clone(), extension).is_some() {
            return Err(ir_error(format!("duplicate resource {}", name.0)));
        }
    }
    Ok(resources)
}

fn extract_security_schemes(
    document: &Value,
) -> Result<BTreeMap<SecuritySchemeName, SecuritySchemeIr>, ExtractError> {
    let mut result = BTreeMap::new();
    let Some(schemes) = document
        .pointer("/components/securitySchemes")
        .and_then(Value::as_object)
    else {
        return Ok(result);
    };
    for (name, scheme) in schemes {
        if scheme.get("$ref").is_some() {
            continue;
        }
        let field = |key: &str| {
            scheme.get(key).and_then(Value::as_str).ok_or_else(|| {
                ExtractError::InvalidOpenApi(format!(
                    "security scheme {name} has no string `{key}`"
                ))
            })
        };
        let scheme = match field("type")? {
            "apiKey" => SecuritySchemeIr::ApiKey {
                name: field("name")?.to_owned(),
                location: match field("in")? {
                    "header" => ApiKeyLocationIr::Header,
                    "query" => ApiKeyLocationIr::Query,
                    "cookie" => ApiKeyLocationIr::Cookie,
                    other => {
                        return Err(ExtractError::InvalidOpenApi(format!(
                            "security scheme {name} has unknown location {other:?}"
                        )));
                    }
                },
            },
            "http" => SecuritySchemeIr::Http {
                scheme: field("scheme")?.to_ascii_lowercase(),
            },
            _ => continue,
        };
        result.insert(SecuritySchemeName(name.clone()), scheme);
    }
    Ok(result)
}

fn response_fields(
    extension: &OperationExtension,
) -> Result<(Option<String>, Option<String>), ExtractError> {
    let Some(response) = &extension.response else {
        return Ok((None, None));
    };
    if response.deleted_when.is_some() && extension.lifecycle != LifecycleAction::Observe {
        return Err(ir_error(format!(
            "resource {}: response.deleted_when is only allowed on the observe operation",
            extension.resource.0
        )));
    }
    Ok((Some(response.path.clone()), response.deleted_when.clone()))
}

fn parse_method(method: &str) -> Result<HttpMethod, ExtractError> {
    match method {
        "get" => Ok(HttpMethod::Get),
        "post" => Ok(HttpMethod::Post),
        "put" => Ok(HttpMethod::Put),
        "patch" => Ok(HttpMethod::Patch),
        "delete" => Ok(HttpMethod::Delete),
        _ => Err(ir_error(format!("unsupported HTTP method {method}"))),
    }
}

fn ir_error(message: impl Into<String>) -> ExtractError {
    ExtractError::Ir(message.into())
}
