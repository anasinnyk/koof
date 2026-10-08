//! The provider IR: resources and operations described by the `x-kube-*`
//! extensions of a prepared `OpenAPI` document ([`extract`]), checked and resolved
//! for code generation by [`validate`].
//!
//! Types that also appear inside the extensions derive `Deserialize`; they are
//! the extensions' schema as well.

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
    /// First `servers[].url` of the document.
    pub base_url: String,
    pub resources: BTreeMap<ResourceName, ResourceIr>,
    pub operations: BTreeMap<OperationId, OperationIr>,
    pub security_schemes: BTreeMap<SecuritySchemeName, SecuritySchemeIr>,
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
    pub scope: ScopeIr,
    pub identifiers: BTreeMap<String, ValueExpr>,
    pub references: BTreeMap<String, ReferenceIr>,
    pub credentials: Option<CredentialsIr>,
    pub lifecycle: LifecycleIr,
    /// CEL rules added to the CRD schema (`x-kubernetes-validations`).
    pub validations: Vec<ValidationIr>,
    /// `forProvider` fields that cannot change or be removed once set.
    pub immutable: Vec<String>,
}

/// `x-kube-resource.scope`.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
pub enum ScopeIr {
    #[default]
    Namespaced,
    Cluster,
}

/// A CEL rule from `x-kube-resource.validations`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ValidationIr {
    pub rule: String,
    pub message: String,
    /// Dot path relative to `spec.forProvider` where the rule applies (`self`);
    /// empty for `forProvider` itself.
    #[serde(default)]
    pub field: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ReferenceIr {
    pub from: FieldPath,
    pub target: ResourceName,
    #[serde(default)]
    pub namespace: NamespacePolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CredentialsIr {
    pub source: CredentialSourceIr,

    pub security_scheme: SecuritySchemeName,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
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

    /// `components.schemas` name of the JSON request body, if any.
    pub request: Option<SchemaName>,
    /// `components.schemas` name of the first successful JSON response, if any.
    pub response: Option<SchemaName>,
    /// Dot path of the resource inside the response body (e.g. `result`).
    pub response_path: Option<String>,
    /// How the request body is built (create / update).
    pub request_mapping: Option<RequestMappingIr>,
    /// Identifiers read from the response, e.g.
    /// `external_id: response.body.result.id`.
    pub result_identifiers: BTreeMap<String, String>,
}

/// `x-kube-operation.request`: the object at `from` plus `set` values.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(transparent)]
pub struct FieldPath(pub String);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamespacePolicy {
    #[default]
    SameAsResource,
    FromReference,
    ClusterScoped,
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec;

    const DOCUMENT: &str = r"
openapi: 3.0.3
info: { title: example, version: '1' }
servers: [{ url: 'https://api.example.com' }]
x-kube-provider: { name: example }
paths:
  /zones:
    post:
      x-kube-operation:
        resource: zone
        lifecycle: create
        result: { identifiers: { external_id: response.body.result.id } }
      requestBody:
        content:
          application/json:
            schema: { type: object, properties: { name: { type: string } } }
      responses:
        '200':
          content:
            application/json:
              schema: { $ref: '#/components/schemas/ZoneEnvelope' }
  /zones/{zone_id}:
    get:
      x-kube-operation:
        resource: zone
        lifecycle: observe
        path_parameters: { zone_id: { type: external_name } }
        response: { path: result }
      responses:
        '200':
          content:
            application/json:
              schema: { $ref: '#/components/schemas/ZoneEnvelope' }
    delete:
      x-kube-operation:
        resource: zone
        lifecycle: delete
        path_parameters: { zone_id: { type: external_name } }
      responses: { '204': { description: gone } }
components:
  securitySchemes:
    token: { type: http, scheme: Bearer }
  schemas:
    ZoneEnvelope:
      type: object
      properties:
        result: { $ref: '#/components/schemas/Zone' }
    Zone:
      x-kube-resource:
        name: zone
        apiVersion: example.kube.io/v1alpha1
        kind: Zone
        identifiers: { external_id: { type: external_name } }
        credentials:
          source: { type: secret_key_selector, path: spec.forProvider.tokenSecretRef }
          security_scheme: token
      type: object
      properties:
        id: { type: string, readOnly: true }
        name: { type: string }
        plan: { type: string, x-kube-field: { late_initialize: false } }
";

    #[test]
    fn prepared_document_yields_a_valid_ir() -> Result<(), Box<dyn std::error::Error>> {
        let prepared = spec::prepare(&serde_yaml::from_str(DOCUMENT)?)?;
        let ir = extract(&prepared)?;

        assert_eq!(ir.provider, "example");
        assert_eq!(ir.base_url, "https://api.example.com");
        assert_eq!(
            ir.security_schemes[&SecuritySchemeName("token".into())],
            SecuritySchemeIr::Http {
                scheme: "bearer".into()
            }
        );
        let zone = &ir.resources[&ResourceName("zone".into())];
        assert_eq!(zone.lifecycle.observe, OperationId("zone.observe".into()));
        assert_eq!(zone.lifecycle.update, None);

        let create = &ir.operations[&OperationId("zone.create".into())];
        assert_eq!(create.method, HttpMethod::Post);
        assert_eq!(create.request, Some(SchemaName("ZoneCreateRequest".into())));
        assert_eq!(create.response, Some(SchemaName("ZoneEnvelope".into())));
        let observe = &ir.operations[&OperationId("zone.observe".into())];
        assert_eq!(observe.response_path.as_deref(), Some("result"));

        let codegen = spec::for_codegen(&prepared)?;
        let provider = validate(&ir, &codegen.late_initialize)
            .map_err(|errors| format!("{errors:?}"))?;
        let [zone] = provider.resources.as_slice() else {
            return Err("expected one resource".into());
        };
        assert_eq!((zone.group, zone.version), ("example.kube.io", "v1alpha1"));
        assert_eq!(
            zone.create.as_ref().map(|create| create.external_name),
            Some("result.id")
        );
        assert_eq!(zone.late_initialize, ["name"]);
        assert_eq!(
            zone.credentials.as_ref().and_then(|credentials| credentials.secret_field),
            Some("tokenSecretRef")
        );
        Ok(())
    }

    #[test]
    fn invalid_ir_reports_every_error() -> Result<(), Box<dyn std::error::Error>> {
        let document = DOCUMENT
            .replace("apiVersion: example.kube.io/v1alpha1", "apiVersion: v1alpha1")
            .replace("result: { identifiers: { external_id: response.body.result.id } }", "");
        let prepared = spec::prepare(&serde_yaml::from_str(&document)?)?;
        let ir = extract(&prepared)?;
        let codegen = spec::for_codegen(&prepared)?;

        let errors = validate(&ir, &codegen.late_initialize)
            .err()
            .ok_or("expected errors")?;
        let messages: Vec<&str> = errors.iter().map(|error| error.message.as_str()).collect();
        assert_eq!(messages.len(), 2, "{messages:?}");
        Ok(())
    }
}
