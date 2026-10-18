//! This file is generated. Do not edit manually.
#![allow(clippy::pedantic)]
/// CRD schema of `spec.forProvider`: unions flattened for kube, then the
/// overlay's CEL validations.
fn for_provider_schema(schema: &mut schemars::Schema) {
    koof::schema::flatten_unions(schema);
    koof::schema::add_validations(schema, &[]);
}
/// `spec.forProvider`: the generated API fields plus references and
/// credentials.
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
#[schemars(transform = for_provider_schema)]
pub struct ClusterOrganizationForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::OrganizationForProvider,
    #[serde(
        rename = "authTokenSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_token_secret_ref: Option<koof::reference::ClusterSecretKeySelector>,
}
#[derive(
    kube::CustomResource,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
#[kube(
    group = "sentry.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "ClusterOrganization",
    status = "ClusterOrganizationStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.sentry\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct ClusterOrganizationSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: ClusterOrganizationForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type ClusterOrganizationStatus = koof::status::ManagedStatus<
    crate::generated::types::OrganizationAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<ClusterOrganization>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "ClusterOrganization",
    api_resource,
    group: "sentry.kube.nas1k.dev",
    namespaced: false,
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    late_initialize: &["name"],
    references: &[],
    credentials: Some(koof::resource::Credentials {
        source: koof::resource::CredentialSource::Secret {
            path: "/spec/forProvider/authTokenSecretRef",
        },
        auth: koof::resource::AuthScheme::Bearer,
    }),
    observe: koof::resource::ObserveDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                ("organization_id_or_slug", koof::resource::ValueExpr::ExternalName),
            ],
            response_path: "",
        },
        deleted_when: None,
    },
    create: None,
    update: Some(koof::resource::UpdateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                ("organization_id_or_slug", koof::resource::ValueExpr::ExternalName),
            ],
            response_path: "",
        },
        request: koof::resource::RequestMapping {
            from: "/spec/forProvider",
            set: &[],
        },
    }),
    delete: None,
};
impl koof::ManagedResource for ClusterOrganization {
    type AtProvider = crate::generated::types::OrganizationAtProvider;
    type Observe = crate::generated::operations::OrganizationObserve;
    type Create = koof::http::Unsupported;
    type Update = crate::generated::operations::OrganizationUpdate;
    type Delete = koof::http::Unsupported;
    fn descriptor() -> &'static koof::ResourceDescriptor {
        &DESCRIPTOR
    }
    fn status(&self) -> Option<&koof::status::ManagedStatus<Self::AtProvider>> {
        self.status.as_ref()
    }
    fn management(&self) -> &koof::managed::ManagedResourceSpec {
        &self.spec.management
    }
}
