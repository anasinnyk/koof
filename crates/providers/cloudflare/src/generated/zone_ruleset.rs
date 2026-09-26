//! This file is generated. Do not edit manually.
#![allow(clippy::pedantic)]
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
#[schemars(transform = koof::schema::flatten_object_unions)]
pub struct RulesetForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::RulesetForProvider,
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
    group = "rulesets.cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "Ruleset",
    namespaced,
    status = "RulesetStatus"
)]
pub struct RulesetSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: RulesetForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type RulesetStatus = koof::status::ManagedStatus<
    crate::generated::types::RulesetAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<Ruleset>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "Ruleset",
    api_resource,
    external_name_annotation: "rulesets.cloudflare.kube.nas1k.dev/external-name",
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    references: &[
        koof::resource::Reference {
            name: "zone",
            from: "spec.forProvider.zoneRef",
            target: &crate::generated::zone::DESCRIPTOR,
            namespace: koof::resource::NamespacePolicy::SameAsResource,
        },
    ],
    credentials: Some(koof::resource::Credentials {
        source: koof::resource::CredentialSource::Related {
            via: &["zone", "account"],
        },
        auth: koof::resource::AuthScheme::Bearer,
    }),
    observe: koof::resource::OperationDescriptor {
        path_parameters: &[
            ("ruleset_id", koof::resource::ValueExpr::ExternalName),
            (
                "zone_id",
                koof::resource::ValueExpr::RelatedIdentifier {
                    via: &["zone"],
                    name: "external_id",
                },
            ),
        ],
        response_path: "result",
    },
};
impl koof::ManagedResource for Ruleset {
    type AtProvider = crate::generated::types::RulesetAtProvider;
    type Observe = crate::generated::operations::ZoneRulesetObserve;
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
