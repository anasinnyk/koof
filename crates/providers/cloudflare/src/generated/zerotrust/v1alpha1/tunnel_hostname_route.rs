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
pub struct TunnelHostnameRouteForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::ZerotrustV1alpha1TunnelHostnameRouteForProvider,
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
    kind = "TunnelHostnameRoute",
    namespaced,
    status = "TunnelHostnameRouteStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.zerotrust\\\\.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct TunnelHostnameRouteSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: TunnelHostnameRouteForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type TunnelHostnameRouteStatus = koof::status::ManagedStatus<
    crate::generated::types::ZerotrustV1alpha1TunnelHostnameRouteAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<TunnelHostnameRoute>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "TunnelHostnameRoute",
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
                            (
                                "hostname_route_id",
                                koof::resource::ValueExpr::ExternalName,
                            ),
                        ],
                    )
                    .with_response_path("/result"),
            )
            .with_deleted_when("/deleted_at"),
    )
    .with_identifiers(&[("external_id", koof::resource::ValueExpr::ExternalName)])
    .with_late_initialize(&["comment", "hostname"])
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
                        ("hostname_route_id", koof::resource::ValueExpr::ExternalName),
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
                    ("hostname_route_id", koof::resource::ValueExpr::ExternalName),
                ],
            ),
    );
impl koof::ManagedResource for TunnelHostnameRoute {
    type AtProvider = crate::generated::types::ZerotrustV1alpha1TunnelHostnameRouteAtProvider;
    type Observe = crate::generated::operations::TunnelHostnameRouteObserve;
    type Create = crate::generated::operations::TunnelHostnameRouteCreate;
    type Update = crate::generated::operations::TunnelHostnameRouteUpdate;
    type Delete = crate::generated::operations::TunnelHostnameRouteDelete;
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
