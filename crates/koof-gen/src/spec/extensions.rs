use super::SpecError;
use crate::ir::{
    CredentialsIr, FieldPath, ReferenceIr, RequestMappingIr, ResourceName, TestIr, ValidationIr,
    ValueExpr,
};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::Value;
use std::collections::BTreeMap;

pub const RESOURCE: &str = "x-kube-resource";
pub const OPERATION: &str = "x-kube-operation";
pub const FIELD: &str = "x-kube-field";
pub const PROVIDER: &str = "x-kube-provider";

pub const HTTP_METHODS: [&str; 8] = [
    "get", "put", "post", "delete", "options", "head", "patch", "trace",
];

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ProviderExtension {
    pub name: String,
    #[serde(default)]
    pub http: HttpExtension,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HttpExtension {
    pub timeout: Option<String>,
    pub connect_timeout: Option<String>,
    pub pool_idle_timeout: Option<String>,
    pub pool_max_idle_per_host: Option<usize>,
    pub user_agent: Option<String>,
}

/// # Errors
///
/// Returns [`SpecError::Extension`] when the extension is malformed.
pub fn kube_provider(document: &Value) -> Result<Option<ProviderExtension>, SpecError> {
    parse(document, PROVIDER, "x-kube-provider")
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResourceExtension {
    pub name: ResourceName,
    pub api_version: String,
    pub kind: String,
    #[serde(default)]
    pub cluster: bool,
    #[serde(default)]
    pub fields: FieldSelection,
    #[serde(default)]
    pub identifiers: BTreeMap<String, ValueExpr>,
    #[serde(default)]
    pub references: BTreeMap<String, ReferenceIr>,
    pub credentials: Option<CredentialsIr>,
    #[serde(default)]
    pub validations: Vec<ValidationIr>,
    #[serde(default)]
    pub immutable: Vec<String>,
    #[serde(default)]
    pub test: Option<TestIr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FieldSelection {
    pub mode: FieldSelectionMode,
    #[serde(default)]
    pub for_provider: Vec<FieldPath>,
    #[serde(default)]
    pub at_provider: Vec<FieldPath>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldSelectionMode {
    #[default]
    All,
    Explicit,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct OperationExtension {
    pub resource: ResourceName,
    pub lifecycle: LifecycleAction,
    #[serde(default)]
    pub path_parameters: BTreeMap<String, ValueExpr>,
    #[serde(default)]
    pub response: Option<ResponseExtension>,
    #[serde(default)]
    pub request: Option<RequestMappingIr>,
    #[serde(default)]
    pub result: Option<ResultExtension>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleAction {
    Observe,
    Create,
    Update,
    Delete,
}

impl LifecycleAction {
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Create => "create",
            Self::Update => "update",
            Self::Delete => "delete",
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResponseExtension {
    pub path: String,
    #[serde(default)]
    pub deleted_when: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ResultExtension {
    #[serde(default)]
    pub identifiers: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FieldExtension {
    #[serde(default)]
    pub ignored: bool,
    #[serde(default = "enabled")]
    pub late_initialize: bool,
    #[serde(default)]
    pub sensitive: bool,
}

fn enabled() -> bool {
    true
}

impl Default for FieldExtension {
    fn default() -> Self {
        Self {
            ignored: false,
            late_initialize: true,
            sensitive: false,
        }
    }
}

impl FieldExtension {
    /// `x-kube-field` of `schema`; the defaults when it has none.
    ///
    /// # Errors
    ///
    /// Returns [`SpecError::Extension`] when the extension is malformed.
    pub fn of(schema: &Value, location: &str) -> Result<Self, SpecError> {
        parse(schema, FIELD, location).map(Option::unwrap_or_default)
    }
}

pub struct KubeOperation<'a> {
    pub path: &'a str,
    pub method: &'a str,
    pub operation: &'a Value,
    pub extension: OperationExtension,
}

impl KubeOperation<'_> {
    #[must_use]
    pub fn location(&self) -> String {
        format!("paths.{}.{}", self.path, self.method)
    }
}

/// Every operation of `document` with `x-kube-operation`, its extension parsed.
///
/// # Errors
///
/// Each item is [`SpecError::Extension`] when its extension is malformed.
pub fn kube_operations(
    document: &Value,
) -> impl Iterator<Item = Result<KubeOperation<'_>, SpecError>> {
    document
        .get("paths")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .filter_map(|(path, item)| Some((path, item.as_object()?)))
        .flat_map(|(path, item)| {
            item.iter()
                .filter(|(method, _)| HTTP_METHODS.contains(&method.as_str()))
                .map(move |(method, operation)| (path, method, operation))
        })
        .filter_map(|(path, method, operation)| {
            let location = format!("paths.{path}.{method}");
            parse::<OperationExtension>(operation, OPERATION, &location)
                .transpose()
                .map(|extension| {
                    extension.map(|extension| KubeOperation {
                        path,
                        method,
                        operation,
                        extension,
                    })
                })
        })
}

pub struct KubeResource<'a> {
    pub schema_name: &'a str,
    pub schema: &'a Value,
    pub extension: ResourceExtension,
}

/// Every component schema of `document` with `x-kube-resource`, its extension parsed.
///
/// # Errors
///
/// Each item is [`SpecError::Extension`] when its extension is malformed.
pub fn kube_resources(
    document: &Value,
) -> impl Iterator<Item = Result<KubeResource<'_>, SpecError>> {
    document
        .pointer("/components/schemas")
        .and_then(Value::as_object)
        .into_iter()
        .flatten()
        .filter_map(|(schema_name, schema)| {
            let location = format!("components.schemas.{schema_name}");
            parse::<ResourceExtension>(schema, RESOURCE, &location)
                .transpose()
                .map(|extension| {
                    extension.map(|extension| KubeResource {
                        schema_name,
                        schema,
                        extension,
                    })
                })
        })
}

fn parse<T: DeserializeOwned>(
    node: &Value,
    name: &'static str,
    location: &str,
) -> Result<Option<T>, SpecError> {
    node.get(name)
        .map(|extension| {
            serde_json::from_value(extension.clone()).map_err(|source| SpecError::Extension {
                extension: name,
                location: location.to_owned(),
                source,
            })
        })
        .transpose()
}
