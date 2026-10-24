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
pub struct LoadBalancerPoolForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::LoadbalancingV1alpha1LoadBalancerPoolForProvider,
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
    group = "loadbalancing.cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "LoadBalancerPool",
    namespaced,
    status = "LoadBalancerPoolStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.loadbalancing\\\\.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct LoadBalancerPoolSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: LoadBalancerPoolForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type LoadBalancerPoolStatus = koof::status::ManagedStatus<
    crate::generated::types::LoadbalancingV1alpha1LoadBalancerPoolAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<LoadBalancerPool>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "LoadBalancerPool",
        api_resource,
        "loadbalancing.cloudflare.kube.nas1k.dev",
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
                        ("pool_id", koof::resource::ValueExpr::ExternalName),
                    ],
                )
                .with_response_path("/result"),
        ),
    )
    .with_identifiers(&[("external_id", koof::resource::ValueExpr::ExternalName)])
    .with_late_initialize(
        &[
            "description",
            "enabled",
            "latitude",
            "load_shedding",
            "longitude",
            "minimum_origins",
            "monitor",
            "monitor_group",
            "name",
            "notification_email",
            "notification_filter",
            "origin_steering",
            "origins",
        ],
    )
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
                        ("pool_id", koof::resource::ValueExpr::ExternalName),
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
                    ("pool_id", koof::resource::ValueExpr::ExternalName),
                ],
            ),
    );
impl koof::ManagedResource for LoadBalancerPool {
    type AtProvider = crate::generated::types::LoadbalancingV1alpha1LoadBalancerPoolAtProvider;
    type Observe = crate::generated::operations::LbPoolObserve;
    type Create = crate::generated::operations::LbPoolCreate;
    type Update = crate::generated::operations::LbPoolUpdate;
    type Delete = crate::generated::operations::LbPoolDelete;
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
