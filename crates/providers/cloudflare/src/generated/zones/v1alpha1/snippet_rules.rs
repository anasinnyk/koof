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
                message: "zoneRef is immutable",
                field: "zoneRef",
            },
            koof::schema::Validation {
                rule: "!has(oldSelf.zoneRef) || has(self.zoneRef)",
                message: "zoneRef cannot be removed once set",
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
pub struct SnippetRulesForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::ZonesV1alpha1SnippetRulesForProvider,
    #[serde(rename = "zoneRef", default, skip_serializing_if = "Option::is_none")]
    pub zone_ref: Option<koof::reference::ResourceReference>,
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
    group = "zones.cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "SnippetRules",
    plural = "snippetrules",
    namespaced,
    status = "SnippetRulesStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct SnippetRulesSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: SnippetRulesForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type SnippetRulesStatus = koof::status::ManagedStatus<
    crate::generated::types::ZonesV1alpha1SnippetRulesAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<SnippetRules>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "SnippetRules",
        api_resource,
        "zones.cloudflare.kube.nas1k.dev",
        true,
        koof::resource::ObserveDescriptor::new(
                koof::resource::OperationDescriptor::new()
                    .with_path_parameters(
                        &[
                            (
                                "zone_id",
                                koof::resource::ValueExpr::RelatedIdentifier {
                                    via: &["zone"],
                                    name: "external_id",
                                },
                            ),
                        ],
                    )
                    .with_response_path("/result"),
            )
            .with_into("rules"),
    )
    .with_late_initialize(&["rules"])
    .with_references(
        &[
            koof::resource::Reference::new(
                "zone",
                "/spec/forProvider/zoneRef",
                &[&crate::generated::cloudflare::v1alpha1::zone::DESCRIPTOR],
            ),
        ],
    )
    .with_credentials(
        koof::resource::Credentials::new(
            koof::resource::CredentialSource::Related {
                via: &["zone", "account"],
            },
            koof::resource::AuthScheme::Bearer,
        ),
    )
    .with_update(
        koof::resource::UpdateDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        (
                            "zone_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["zone"],
                                name: "external_id",
                            },
                        ),
                    ],
                ),
            koof::resource::RequestMapping::new("/spec/forProvider"),
        ),
    );
impl koof::ManagedResource for SnippetRules {
    type AtProvider = crate::generated::types::ZonesV1alpha1SnippetRulesAtProvider;
    type Observe = crate::generated::operations::SnippetRulesObserve;
    type Create = koof::http::Unsupported;
    type Update = crate::generated::operations::SnippetRulesUpdate;
    type Delete = koof::http::Unsupported;
    type Connection = koof::http::Unsupported;
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
