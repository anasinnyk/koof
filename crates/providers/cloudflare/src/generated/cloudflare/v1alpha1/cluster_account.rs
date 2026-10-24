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
pub struct ClusterAccountForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::CloudflareV1alpha1AccountForProvider,
    #[serde(
        rename = "apiTokenSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_token_secret_ref: Option<koof::reference::ClusterSecretKeySelector>,
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
    group = "cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "ClusterAccount",
    status = "ClusterAccountStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct ClusterAccountSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: ClusterAccountForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type ClusterAccountStatus = koof::status::ManagedStatus<
    crate::generated::types::CloudflareV1alpha1AccountAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<ClusterAccount>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "ClusterAccount",
        api_resource,
        "cloudflare.kube.nas1k.dev",
        false,
        koof::resource::ObserveDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[("account_id", koof::resource::ValueExpr::ExternalName)],
                )
                .with_response_path("/result"),
        ),
    )
    .with_identifiers(&[("external_id", koof::resource::ValueExpr::ExternalName)])
    .with_late_initialize(&["name", "settings"])
    .with_credentials(
        koof::resource::Credentials::new(
            koof::resource::CredentialSource::Secret {
                path: "/spec/forProvider/apiTokenSecretRef",
            },
            koof::resource::AuthScheme::Bearer,
        ),
    )
    .with_create(
        koof::resource::CreateDescriptor::new(
            koof::resource::OperationDescriptor::new(),
            koof::resource::RequestMapping::new("/spec/forProvider"),
            "/result/id",
        ),
    )
    .with_update(
        koof::resource::UpdateDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[("account_id", koof::resource::ValueExpr::ExternalName)],
                ),
            koof::resource::RequestMapping::new("/spec/forProvider")
                .with_set(
                    &[
                        ("/id", koof::resource::ValueExpr::ExternalName),
                        (
                            "/type",
                            koof::resource::ValueExpr::Field("/status/atProvider/type"),
                        ),
                    ],
                ),
        ),
    )
    .with_delete(
        koof::resource::OperationDescriptor::new()
            .with_path_parameters(
                &[("account_id", koof::resource::ValueExpr::ExternalName)],
            ),
    );
impl koof::ManagedResource for ClusterAccount {
    type AtProvider = crate::generated::types::CloudflareV1alpha1AccountAtProvider;
    type Observe = crate::generated::operations::AccountObserve;
    type Create = crate::generated::operations::AccountCreate;
    type Update = crate::generated::operations::AccountUpdate;
    type Delete = crate::generated::operations::AccountDelete;
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
