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
pub struct TunnelCidrRouteForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::TunnelCidrRouteForProvider,
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
    crate::generated::types::TunnelCidrRouteAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<TunnelCidrRoute>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "TunnelCidrRoute",
    api_resource,
    group: "zerotrust.cloudflare.kube.nas1k.dev",
    namespaced: true,
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    late_initialize: &["comment", "network", "virtual_network_id"],
    references: &[
        koof::resource::Reference {
            name: "tunnel",
            from: "/spec/forProvider/tunnelRef",
            targets: &[&crate::generated::tunnel::DESCRIPTOR],
        },
    ],
    credentials: Some(koof::resource::Credentials {
        source: koof::resource::CredentialSource::Related {
            via: &["tunnel", "account"],
        },
        auth: koof::resource::AuthScheme::Bearer,
    }),
    observe: koof::resource::ObserveDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                (
                    "account_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["tunnel", "account"],
                        name: "external_id",
                    },
                ),
                ("route_id", koof::resource::ValueExpr::ExternalName),
            ],
            response_path: "/result",
        },
        deleted_when: Some("/deleted_at"),
    },
    create: Some(koof::resource::CreateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                (
                    "account_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["tunnel", "account"],
                        name: "external_id",
                    },
                ),
            ],
            response_path: "",
        },
        request: koof::resource::RequestMapping {
            from: "/spec/forProvider",
            set: &[
                (
                    "/tunnel_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["tunnel"],
                        name: "external_id",
                    },
                ),
            ],
        },
        external_name: "/result/id",
    }),
    update: Some(koof::resource::UpdateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                (
                    "account_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["tunnel", "account"],
                        name: "external_id",
                    },
                ),
                ("route_id", koof::resource::ValueExpr::ExternalName),
            ],
            response_path: "",
        },
        request: koof::resource::RequestMapping {
            from: "/spec/forProvider",
            set: &[
                (
                    "/tunnel_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["tunnel"],
                        name: "external_id",
                    },
                ),
            ],
        },
    }),
    delete: Some(koof::resource::OperationDescriptor {
        path_parameters: &[
            (
                "account_id",
                koof::resource::ValueExpr::RelatedIdentifier {
                    via: &["tunnel", "account"],
                    name: "external_id",
                },
            ),
            ("route_id", koof::resource::ValueExpr::ExternalName),
        ],
        response_path: "",
    }),
};
impl koof::ManagedResource for TunnelCidrRoute {
    type AtProvider = crate::generated::types::TunnelCidrRouteAtProvider;
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
