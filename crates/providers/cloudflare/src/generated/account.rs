//! This file is generated. Do not edit manually.
#![allow(clippy::pedantic)]
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
#[schemars(transform = koof::schema::flatten_object_unions)]
pub struct AccountForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::AccountForProvider,
    #[serde(
        rename = "apiTokenSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_token_secret_ref: Option<koof::reference::SecretReference>,
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
    kind = "Account",
    namespaced,
    status = "AccountStatus"
)]
pub struct AccountSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: AccountForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type AccountStatus = koof::status::ManagedStatus<
    crate::generated::types::AccountAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<Account>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "Account",
    api_resource,
    identifiers: &[
        ("external_id", koof::resource::ValueExpr::Field("status.atProvider.id")),
    ],
    references: &[],
    credentials: Some(koof::resource::Credentials {
        source: koof::resource::CredentialSource::Secret {
            path: "spec.forProvider.apiTokenSecretRef",
        },
        auth: koof::resource::AuthScheme::Bearer,
    }),
    observe: koof::resource::OperationDescriptor {
        path_parameters: &[
            ("account_id", koof::resource::ValueExpr::Field("status.atProvider.id")),
        ],
        response_path: "result",
    },
};
impl koof::ManagedResource for Account {
    type AtProvider = crate::generated::types::AccountAtProvider;
    type Observe = crate::generated::operations::AccountObserve;
    fn descriptor() -> &'static koof::ResourceDescriptor {
        &DESCRIPTOR
    }
    fn status(&self) -> Option<&koof::status::ManagedStatus<Self::AtProvider>> {
        self.status.as_ref()
    }
}
