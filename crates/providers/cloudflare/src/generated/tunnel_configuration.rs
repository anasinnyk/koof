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
pub struct TunnelConfigurationForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::TunnelConfigurationForProvider,
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
    kind = "TunnelConfiguration",
    namespaced,
    status = "TunnelConfigurationStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct TunnelConfigurationSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: TunnelConfigurationForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type TunnelConfigurationStatus = koof::status::ManagedStatus<
    crate::generated::types::TunnelConfigurationAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<TunnelConfiguration>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "TunnelConfiguration",
    api_resource,
    group: "zerotrust.cloudflare.kube.nas1k.dev",
    namespaced: true,
    identifiers: &[],
    late_initialize: &["config"],
    references: &[
        koof::resource::Reference {
            name: "tunnel",
            from: "/spec/forProvider/tunnelRef",
            target: &crate::generated::tunnel::DESCRIPTOR,
            namespace: koof::resource::NamespacePolicy::SameAsResource,
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
                (
                    "tunnel_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["tunnel"],
                        name: "external_id",
                    },
                ),
            ],
            response_path: "/result",
        },
        deleted_when: None,
    },
    create: None,
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
                (
                    "tunnel_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["tunnel"],
                        name: "external_id",
                    },
                ),
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
impl koof::ManagedResource for TunnelConfiguration {
    type AtProvider = crate::generated::types::TunnelConfigurationAtProvider;
    type Observe = crate::generated::operations::TunnelConfigurationObserve;
    type Create = koof::http::Unsupported;
    type Update = crate::generated::operations::TunnelConfigurationUpdate;
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
