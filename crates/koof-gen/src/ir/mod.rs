mod extract;
pub mod resolved;
mod validate;

pub use extract::{ExtractError, extract};
pub use validate::{ValidationError, validate};

use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderIr {
    pub provider: String,
    pub base_url: String,
    pub http: HttpIr,
    pub resources: BTreeMap<ResourceName, ResourceIr>,
    pub operations: BTreeMap<OperationId, OperationIr>,
    pub security_schemes: BTreeMap<SecuritySchemeName, SecuritySchemeIr>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HttpIr {
    pub timeout: Option<std::time::Duration>,
    pub connect_timeout: Option<std::time::Duration>,
    pub pool_idle_timeout: Option<std::time::Duration>,
    pub pool_max_idle_per_host: Option<usize>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(transparent)]
pub struct ResourceName(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OperationId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceIr {
    pub api_version: String,
    pub kind: String,
    pub cluster: bool,
    pub identifiers: BTreeMap<String, ValueExpr>,
    pub references: BTreeMap<String, ReferenceIr>,
    pub credentials: Option<CredentialsIr>,
    pub lifecycle: LifecycleIr,
    pub validations: Vec<ValidationIr>,
    pub immutable: Vec<String>,
    pub test: Option<TestIr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TestIr {
    #[serde(default)]
    pub for_provider: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    pub update: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ValidationIr {
    pub rule: String,
    pub message: String,
    #[serde(default)]
    pub field: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReferenceIr {
    pub from: FieldPath,
    pub target: ResourceName,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CredentialsIr {
    pub source: CredentialSourceIr,

    pub security_scheme: SecuritySchemeName,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum CredentialSourceIr {
    SecretKeySelector { path: FieldPath },
    Related { via: Vec<String> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifecycleIr {
    pub observe: OperationId,
    pub create: Option<OperationId>,
    pub update: Option<OperationId>,
    pub delete: Option<OperationId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationIr {
    pub method: HttpMethod,
    pub path: String,
    pub path_parameters: BTreeMap<String, ValueExpr>,

    pub request: Option<SchemaName>,
    pub response: Option<SchemaName>,
    pub response_path: Option<String>,
    pub deleted_when: Option<String>,
    pub request_mapping: Option<RequestMappingIr>,
    pub result_identifiers: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RequestMappingIr {
    #[serde(default = "RequestMappingIr::default_from")]
    pub from: FieldPath,
    #[serde(default)]
    pub set: BTreeMap<String, ValueExpr>,
}

impl RequestMappingIr {
    fn default_from() -> FieldPath {
        FieldPath("spec.forProvider".to_owned())
    }
}

impl Default for RequestMappingIr {
    fn default() -> Self {
        Self {
            from: Self::default_from(),
            set: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum ValueExpr {
    Field { path: FieldPath },
    ExternalName,
    RelatedIdentifier { via: Vec<String>, name: String },
    Coalesce { values: Vec<ValueExpr> },
    Literal { value: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct FieldPath(pub String);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SchemaName(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub struct SecuritySchemeName(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiKeyLocationIr {
    Header,
    Query,
    Cookie,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecuritySchemeIr {
    ApiKey {
        name: String,
        location: ApiKeyLocationIr,
    },

    Http {
        scheme: String,
    },
}

/// Module name of an API group: its first DNS label in snake case.
#[must_use]
pub fn group_module(group: &str) -> String {
    heck::ToSnakeCase::to_snake_case(group.split('.').next().unwrap_or(group))
}

/// `components.schemas` key of a synthesized schema (`role` is `ForProvider` or
/// `AtProvider`): [`synthesized_prefix`] followed by `<kind><role>`.
#[must_use]
pub fn synthesized_schema(api_version: &str, kind: &str, role: &str) -> String {
    format!("{}{kind}{role}", synthesized_prefix(api_version))
}

/// `<group module>_<version>_`, the prefix of the synthesized schemas of the
/// resources of `api_version`.
#[must_use]
pub fn synthesized_prefix(api_version: &str) -> String {
    let (group, version) = api_version.split_once('/').unwrap_or(("", api_version));
    format!("{}_{version}_", group_module(group))
}
