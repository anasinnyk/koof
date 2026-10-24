//! This file is generated. Do not edit manually.
#![allow(clippy::pedantic)]
/// CRD schema of `spec.forProvider`: unions flattened for kube, then the
/// overlay's CEL validations.
fn for_provider_schema(schema: &mut schemars::Schema) {
    koof::schema::flatten_unions(schema);
    koof::schema::add_validations(
        schema,
        &[
            koof::schema::Validation {
                rule: "self == oldSelf",
                message: "accountRef is immutable",
                field: "accountRef",
            },
            koof::schema::Validation {
                rule: "!has(oldSelf.accountRef) || has(self.accountRef)",
                message: "accountRef cannot be removed once set",
                field: "",
            },
        ],
    );
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
pub struct GatewayListForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::ZerotrustV1alpha1GatewayListForProvider,
    #[serde(rename = "accountRef", default, skip_serializing_if = "Option::is_none")]
    pub account_ref: Option<
        crate::generated::cloudflare::v1alpha1::account::AccountReference,
    >,
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
    group = "zerotrust.cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "GatewayList",
    namespaced,
    status = "GatewayListStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.zerotrust\\\\.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct GatewayListSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: GatewayListForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type GatewayListStatus = koof::status::ManagedStatus<
    crate::generated::types::ZerotrustV1alpha1GatewayListAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<GatewayList>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "GatewayList",
        api_resource,
        "zerotrust.cloudflare.kube.nas1k.dev",
        true,
        koof::resource::ObserveDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        (
                            "account_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["account"],
                                name: "external_id",
                            },
                        ),
                        ("list_id", koof::resource::ValueExpr::ExternalName),
                    ],
                )
                .with_response_path("/result"),
        ),
    )
    .with_identifiers(&[("external_id", koof::resource::ValueExpr::ExternalName)])
    .with_late_initialize(&["description", "items", "name", "type"])
    .with_references(
        &[
            koof::resource::Reference::new(
                "account",
                "/spec/forProvider/accountRef",
                &[
                    &crate::generated::cloudflare::v1alpha1::account::DESCRIPTOR,
                    &crate::generated::cloudflare::v1alpha1::cluster_account::DESCRIPTOR,
                ],
            ),
        ],
    )
    .with_credentials(
        koof::resource::Credentials::new(
            koof::resource::CredentialSource::Related {
                via: &["account"],
            },
            koof::resource::AuthScheme::Bearer,
        ),
    )
    .with_create(
        koof::resource::CreateDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        (
                            "account_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["account"],
                                name: "external_id",
                            },
                        ),
                    ],
                ),
            koof::resource::RequestMapping::new("/spec/forProvider"),
            "/result/id",
        ),
    )
    .with_update(
        koof::resource::UpdateDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        (
                            "account_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["account"],
                                name: "external_id",
                            },
                        ),
                        ("list_id", koof::resource::ValueExpr::ExternalName),
                    ],
                ),
            koof::resource::RequestMapping::new("/spec/forProvider"),
        ),
    )
    .with_delete(
        koof::resource::OperationDescriptor::new()
            .with_path_parameters(
                &[
                    (
                        "account_id",
                        koof::resource::ValueExpr::RelatedIdentifier {
                            via: &["account"],
                            name: "external_id",
                        },
                    ),
                    ("list_id", koof::resource::ValueExpr::ExternalName),
                ],
            ),
    );
impl koof::ManagedResource for GatewayList {
    type AtProvider = crate::generated::types::ZerotrustV1alpha1GatewayListAtProvider;
    type Observe = crate::generated::operations::GatewayListObserve;
    type Create = crate::generated::operations::GatewayListCreate;
    type Update = crate::generated::operations::GatewayListUpdate;
    type Delete = crate::generated::operations::GatewayListDelete;
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
