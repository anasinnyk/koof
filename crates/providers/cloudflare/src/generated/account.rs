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
pub struct AccountForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::AccountForProvider,
    #[serde(
        rename = "apiTokenSecretRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub api_token_secret_ref: Option<koof::reference::SecretKeySelector>,
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
    status = "AccountStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct AccountSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: AccountForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
///A reference to an object of kind `Account` (default) in the namespace of the resource holding the reference, or of kind `ClusterAccount`, by `kind`.
#[derive(
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema
)]
pub struct AccountReference {
    pub name: String,
    #[serde(default)]
    pub kind: AccountReferenceKind,
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
pub enum AccountReferenceKind {
    #[default]
    Account,
    ClusterAccount,
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
    group: "cloudflare.kube.nas1k.dev",
    namespaced: true,
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    late_initialize: &["name", "settings"],
    references: &[],
    credentials: Some(koof::resource::Credentials {
        source: koof::resource::CredentialSource::Secret {
            path: "/spec/forProvider/apiTokenSecretRef",
        },
        auth: koof::resource::AuthScheme::Bearer,
    }),
    observe: koof::resource::ObserveDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[("account_id", koof::resource::ValueExpr::ExternalName)],
            response_path: "/result",
        },
        deleted_when: None,
    },
    create: Some(koof::resource::CreateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[],
            response_path: "",
        },
        request: koof::resource::RequestMapping {
            from: "/spec/forProvider",
            set: &[],
        },
        external_name: "/result/id",
    }),
    update: Some(koof::resource::UpdateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[("account_id", koof::resource::ValueExpr::ExternalName)],
            response_path: "",
        },
        request: koof::resource::RequestMapping {
            from: "/spec/forProvider",
            set: &[
                ("/id", koof::resource::ValueExpr::ExternalName),
                ("/type", koof::resource::ValueExpr::Field("/status/atProvider/type")),
            ],
        },
    }),
    delete: Some(koof::resource::OperationDescriptor {
        path_parameters: &[("account_id", koof::resource::ValueExpr::ExternalName)],
        response_path: "",
    }),
};
impl koof::ManagedResource for Account {
    type AtProvider = crate::generated::types::AccountAtProvider;
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
