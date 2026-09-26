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
pub struct GatewayRuleForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::GatewayRuleForProvider,
    #[serde(rename = "accountRef", default, skip_serializing_if = "Option::is_none")]
    pub account_ref: Option<koof::reference::ResourceReference>,
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
    kind = "GatewayRule",
    namespaced,
    status = "GatewayRuleStatus"
)]
pub struct GatewayRuleSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: GatewayRuleForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type GatewayRuleStatus = koof::status::ManagedStatus<
    crate::generated::types::GatewayRuleAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<GatewayRule>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "GatewayRule",
    api_resource,
    group: "zerotrust.cloudflare.kube.nas1k.dev",
    namespaced: true,
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    late_initialize: &[
        "action",
        "description",
        "device_posture",
        "enabled",
        "expiration",
        "filters",
        "identity",
        "name",
        "precedence",
        "rule_settings",
        "schedule",
        "traffic",
    ],
    references: &[
        koof::resource::Reference {
            name: "account",
            from: "spec.forProvider.accountRef",
            target: &crate::generated::account::DESCRIPTOR,
            namespace: koof::resource::NamespacePolicy::SameAsResource,
        },
    ],
    credentials: Some(koof::resource::Credentials {
        source: koof::resource::CredentialSource::Related {
            via: &["account"],
        },
        auth: koof::resource::AuthScheme::Bearer,
    }),
    observe: koof::resource::OperationDescriptor {
        path_parameters: &[
            (
                "account_id",
                koof::resource::ValueExpr::RelatedIdentifier {
                    via: &["account"],
                    name: "external_id",
                },
            ),
            ("rule_id", koof::resource::ValueExpr::ExternalName),
        ],
        response_path: "result",
    },
    create: Some(koof::resource::CreateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                (
                    "account_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["account"],
                        name: "external_id",
                    },
                ),
            ],
            response_path: "",
        },
        request: koof::resource::RequestMapping {
            from: "spec.forProvider",
            set: &[],
        },
        external_name: "result.id",
    }),
    update: Some(koof::resource::UpdateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                (
                    "account_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["account"],
                        name: "external_id",
                    },
                ),
                ("rule_id", koof::resource::ValueExpr::ExternalName),
            ],
            response_path: "",
        },
        request: koof::resource::RequestMapping {
            from: "spec.forProvider",
            set: &[],
        },
    }),
    delete: Some(koof::resource::OperationDescriptor {
        path_parameters: &[
            (
                "account_id",
                koof::resource::ValueExpr::RelatedIdentifier {
                    via: &["account"],
                    name: "external_id",
                },
            ),
            ("rule_id", koof::resource::ValueExpr::ExternalName),
        ],
        response_path: "",
    }),
};
impl koof::ManagedResource for GatewayRule {
    type AtProvider = crate::generated::types::GatewayRuleAtProvider;
    type Observe = crate::generated::operations::GatewayRuleObserve;
    type Create = crate::generated::operations::GatewayRuleCreate;
    type Update = crate::generated::operations::GatewayRuleUpdate;
    type Delete = crate::generated::operations::GatewayRuleDelete;
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
