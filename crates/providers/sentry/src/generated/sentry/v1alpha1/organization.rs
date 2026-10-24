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
pub struct OrganizationForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::SentryV1alpha1OrganizationForProvider,
    #[serde(
        rename = "authTokenSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auth_token_secret_ref: Option<koof::reference::SecretKeySelector>,
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
    kind = "Organization",
    namespaced,
    status = "OrganizationStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.sentry\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct OrganizationSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: OrganizationForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
///A reference to an object of kind `Organization` (default) in the namespace of the resource holding the reference, or of kind `ClusterOrganization`, by `kind`.
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct OrganizationReference {
    pub name: String,
    #[serde(default)]
    pub kind: OrganizationReferenceKind,
    /// `apiVersion` of the referenced object; needed when its kind is in several groups.
    #[serde(rename = "apiVersion", default, skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
}
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub enum OrganizationReferenceKind {
    #[default]
    Organization,
    ClusterOrganization,
}
pub type OrganizationStatus = koof::status::ManagedStatus<
    crate::generated::types::SentryV1alpha1OrganizationAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<Organization>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "Organization",
        api_resource,
        "sentry.kube.nas1k.dev",
        true,
        koof::resource::ObserveDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        (
                            "organization_id_or_slug",
                            koof::resource::ValueExpr::ExternalName,
                        ),
                    ],
                ),
        ),
    )
    .with_identifiers(&[("external_id", koof::resource::ValueExpr::ExternalName)])
    .with_late_initialize(&["name"])
    .with_credentials(
        koof::resource::Credentials::new(
            koof::resource::CredentialSource::Secret {
                path: "/spec/forProvider/authTokenSecretRef",
            },
            koof::resource::AuthScheme::Bearer,
        ),
    )
    .with_update(
        koof::resource::UpdateDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        (
                            "organization_id_or_slug",
                            koof::resource::ValueExpr::ExternalName,
                        ),
                    ],
                ),
            koof::resource::RequestMapping::new("/spec/forProvider"),
        ),
    );
impl koof::ManagedResource for Organization {
    type AtProvider = crate::generated::types::SentryV1alpha1OrganizationAtProvider;
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
