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
pub struct EmailRoutingRuleForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::EmailRoutingRuleForProvider,
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
    kind = "EmailRoutingRule",
    namespaced,
    status = "EmailRoutingRuleStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.zones\\\\.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct EmailRoutingRuleSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: EmailRoutingRuleForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type EmailRoutingRuleStatus = koof::status::ManagedStatus<
    crate::generated::types::EmailRoutingRuleAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<EmailRoutingRule>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "EmailRoutingRule",
        api_resource,
        "zones.cloudflare.kube.nas1k.dev",
        true,
        koof::resource::ObserveDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        ("rule_identifier", koof::resource::ValueExpr::ExternalName),
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
        ),
    )
    .with_identifiers(&[("external_id", koof::resource::ValueExpr::ExternalName)])
    .with_late_initialize(
        &[
            "actions",
            "enabled",
            "matchers",
            "name",
            "owner_worker_tag",
            "priority",
            "source",
        ],
    )
    .with_references(
        &[
            koof::resource::Reference::new(
                "zone",
                "/spec/forProvider/zoneRef",
                &[&crate::generated::zone::DESCRIPTOR],
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
    .with_create(
        koof::resource::CreateDescriptor::new(
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
            "/result/id",
        ),
    )
    .with_update(
        koof::resource::UpdateDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        ("rule_identifier", koof::resource::ValueExpr::ExternalName),
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
    )
    .with_delete(
        koof::resource::OperationDescriptor::new()
            .with_path_parameters(
                &[
                    ("rule_identifier", koof::resource::ValueExpr::ExternalName),
                    (
                        "zone_id",
                        koof::resource::ValueExpr::RelatedIdentifier {
                            via: &["zone"],
                            name: "external_id",
                        },
                    ),
                ],
            ),
    );
impl koof::ManagedResource for EmailRoutingRule {
    type AtProvider = crate::generated::types::EmailRoutingRuleAtProvider;
    type Observe = crate::generated::operations::EmailRoutingRuleObserve;
    type Create = crate::generated::operations::EmailRoutingRuleCreate;
    type Update = crate::generated::operations::EmailRoutingRuleUpdate;
    type Delete = crate::generated::operations::EmailRoutingRuleDelete;
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
