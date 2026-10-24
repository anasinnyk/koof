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
                message: "tunnelRef is immutable",
                field: "tunnelRef",
            },
            koof::schema::Validation {
                rule: "!has(oldSelf.tunnelRef) || has(self.tunnelRef)",
                message: "tunnelRef cannot be removed once set",
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
pub struct TunnelCidrRouteForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::ZerotrustV1alpha1TunnelCidrRouteForProvider,
    #[serde(rename = "tunnelRef", default, skip_serializing_if = "Option::is_none")]
    pub tunnel_ref: Option<koof::reference::ResourceReference>,
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
    kind = "TunnelCidrRoute",
    namespaced,
    status = "TunnelCidrRouteStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.zerotrust\\\\.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct TunnelCidrRouteSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: TunnelCidrRouteForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type TunnelCidrRouteStatus = koof::status::ManagedStatus<
    crate::generated::types::ZerotrustV1alpha1TunnelCidrRouteAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<TunnelCidrRoute>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "TunnelCidrRoute",
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
                                    via: &["tunnel", "account"],
                                    name: "external_id",
                                },
                            ),
                            ("route_id", koof::resource::ValueExpr::ExternalName),
                        ],
                    )
                    .with_response_path("/result"),
            )
            .with_deleted_when("/deleted_at"),
    )
    .with_identifiers(&[("external_id", koof::resource::ValueExpr::ExternalName)])
    .with_late_initialize(&["comment", "network", "virtual_network_id"])
    .with_references(
        &[
            koof::resource::Reference::new(
                "tunnel",
                "/spec/forProvider/tunnelRef",
                &[&crate::generated::zerotrust::v1alpha1::tunnel::DESCRIPTOR],
            ),
        ],
    )
    .with_credentials(
        koof::resource::Credentials::new(
            koof::resource::CredentialSource::Related {
                via: &["tunnel", "account"],
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
                                via: &["tunnel", "account"],
                                name: "external_id",
                            },
                        ),
                    ],
                ),
            koof::resource::RequestMapping::new("/spec/forProvider")
                .with_set(
                    &[
                        (
                            "/tunnel_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["tunnel"],
                                name: "external_id",
                            },
                        ),
                    ],
                ),
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
                                via: &["tunnel", "account"],
                                name: "external_id",
                            },
                        ),
                        ("route_id", koof::resource::ValueExpr::ExternalName),
                    ],
                ),
            koof::resource::RequestMapping::new("/spec/forProvider")
                .with_set(
                    &[
                        (
                            "/tunnel_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["tunnel"],
                                name: "external_id",
                            },
                        ),
                    ],
                ),
        ),
    )
    .with_delete(
        koof::resource::OperationDescriptor::new()
            .with_path_parameters(
                &[
                    (
                        "account_id",
                        koof::resource::ValueExpr::RelatedIdentifier {
                            via: &["tunnel", "account"],
                            name: "external_id",
                        },
                    ),
                    ("route_id", koof::resource::ValueExpr::ExternalName),
                ],
            ),
    );
impl koof::ManagedResource for TunnelCidrRoute {
    type AtProvider = crate::generated::types::ZerotrustV1alpha1TunnelCidrRouteAtProvider;
    type Observe = crate::generated::operations::TunnelCidrRouteObserve;
    type Create = crate::generated::operations::TunnelCidrRouteCreate;
    type Update = crate::generated::operations::TunnelCidrRouteUpdate;
    type Delete = crate::generated::operations::TunnelCidrRouteDelete;
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
