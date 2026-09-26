use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderIr {
    pub version: u32,
    pub provider: String,
    /// First `servers[].url` of the document.
    #[serde(default)]
    pub base_url: String,
    pub resources: BTreeMap<ResourceName, ResourceIr>,
    pub operations: BTreeMap<OperationId, OperationIr>,
    pub security_schemes: BTreeMap<SecuritySchemeName, SecuritySchemeIr>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourceName(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct OperationId(pub String);

impl OperationId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for OperationId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldSelectionModeIr {
    All,
    Explicit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FieldSelectionIr {
    pub mode: FieldSelectionModeIr,
    #[serde(default)]
    pub for_provider: Vec<FieldPath>,
    #[serde(default)]
    pub at_provider: Vec<FieldPath>,
}

impl Default for FieldSelectionIr {
    fn default() -> Self {
        Self {
            mode: FieldSelectionModeIr::All,
            for_provider: Vec::new(),
            at_provider: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceIr {
    pub api_version: String,
    pub kind: String,
    #[serde(default)]
    pub scope: ScopeIr,
    #[serde(default)]
    pub identifiers: BTreeMap<String, ValueExpr>,
    #[serde(default)]
    pub field_selection: FieldSelectionIr,
    #[serde(default)]
    pub references: BTreeMap<String, ReferenceIr>,
    pub credentials: Option<CredentialsIr>,
    pub lifecycle: LifecycleIr,
    /// CEL rules added to the CRD schema (`x-kubernetes-validations`).
    #[serde(default)]
    pub validations: Vec<ValidationIr>,
    /// `forProvider` fields that cannot change or be removed once set.
    #[serde(default)]
    pub immutable: Vec<String>,
}

/// `x-kube-resource.scope`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScopeIr {
    #[default]
    Namespaced,
    Cluster,
}

/// A CEL rule from `x-kube-resource.validations`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationIr {
    pub rule: String,
    pub message: String,
    /// Dot path relative to `spec.forProvider` where the rule applies (`self`);
    /// empty for `forProvider` itself.
    #[serde(default)]
    pub field: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceIr {
    pub from: FieldPath,
    pub target: ResourceName,
    #[serde(default)]
    pub namespace: NamespacePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CredentialsIr {
    pub source: CredentialSourceIr,

    pub security_scheme: SecuritySchemeName,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CredentialSourceIr {
    SecretKeySelector { path: FieldPath },
    Related { via: Vec<String> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleIr {
    pub observe: OperationId,
    pub create: Option<OperationId>,
    pub update: Option<OperationId>,
    pub delete: Option<OperationId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationIr {
    pub method: HttpMethod,
    pub path: String,
    #[serde(default)]
    pub path_parameters: BTreeMap<String, ValueExpr>,

    /// `components.schemas` name of the JSON request body, if any.
    #[serde(default)]
    pub request: Option<SchemaName>,
    /// `components.schemas` name of the first successful JSON response, if any.
    #[serde(default)]
    pub response: Option<SchemaName>,
    /// Dot path of the resource inside the response body (e.g. `result`).
    #[serde(default)]
    pub response_path: Option<String>,
    /// How the request body is built (create / update).
    #[serde(default)]
    pub request_mapping: Option<RequestMappingIr>,
    /// Identifiers read from the response, e.g.
    /// `external_id: response.body.result.id`.
    #[serde(default)]
    pub result_identifiers: BTreeMap<String, String>,
}

/// `x-kube-operation.request`: the object at `from` plus `set` values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ValueExpr {
    Field {
        path: FieldPath,
    },
    /// The resource's external name annotation (`<group>/external-name`).
    ExternalName,
    RelatedIdentifier {
        via: Vec<String>,
        name: String,
    },
    Coalesce {
        values: Vec<ValueExpr>,
    },
    Literal {
        value: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FieldPath(pub String);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamespacePolicy {
    #[default]
    SameAsResource,
    FromReference,
    ClusterScoped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

impl HttpMethod {
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethod::GET => "get",
            HttpMethod::POST => "post",
            HttpMethod::PUT => "put",
            HttpMethod::PATCH => "patch",
            HttpMethod::DELETE => "delete",
        }
    }
}

impl AsRef<str> for HttpMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaName(pub String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SecuritySchemeName(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiKeyLocationIr {
    Header,
    Query,
    Cookie,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecuritySchemeIr {
    ApiKey {
        name: String,
        location: ApiKeyLocationIr,
    },

    Http {
        scheme: String,
        bearer_format: Option<String>,
    },
}

#[cfg(test)]
mod test {
    use super::*;

    const FIXTURE: &str = include_str!("fixtures/minimal-provider.yaml");

    #[test]
    fn minimal_provider_fixture_describes_the_ir() -> Result<(), serde_yaml::Error> {
        let ir: ProviderIr = serde_yaml::from_str(FIXTURE)?;

        assert_eq!(ir.version, 1);
        assert_eq!(ir.provider, "cloudflare");
        assert_eq!(ir.resources.len(), 3);

        let dns_record = &ir.resources[&ResourceName("dns_record".into())];
        assert_eq!(dns_record.kind, "DNSRecord");
        assert_eq!(dns_record.references["zone"].target.0, "zone");
        assert_eq!(
            dns_record
                .credentials
                .as_ref()
                .map(|credentials| &credentials.source),
            Some(&CredentialSourceIr::Related {
                via: vec!["zone".to_string(), "account".to_string()]
            })
        );
        assert_eq!(
            dns_record.field_selection.mode,
            FieldSelectionModeIr::Explicit
        );
        assert_eq!(dns_record.field_selection.for_provider.len(), 2);

        let account = &ir.resources[&ResourceName("account".into())];
        assert_eq!(account.field_selection, FieldSelectionIr::default());

        let get = &ir.operations[&OperationId("dns_record.get".into())];
        assert_eq!(get.method, HttpMethod::GET);
        assert_eq!(get.path_parameters.len(), 2);

        let yaml = serde_yaml::to_string(&ir)?;
        let decoded: ProviderIr = serde_yaml::from_str(&yaml)?;
        assert_eq!(decoded, ir);

        Ok(())
    }
}
