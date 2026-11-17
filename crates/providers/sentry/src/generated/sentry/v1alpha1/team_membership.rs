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
                message: "memberRef is immutable",
                field: "memberRef",
            },
            koof::schema::Validation {
                rule: "!has(oldSelf.memberRef) || has(self.memberRef)",
                message: "memberRef cannot be removed once set",
                field: "",
            },
            koof::schema::Validation {
                rule: "self == oldSelf",
                message: "teamRef is immutable",
                field: "teamRef",
            },
            koof::schema::Validation {
                rule: "!has(oldSelf.teamRef) || has(self.teamRef)",
                message: "teamRef cannot be removed once set",
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
pub struct TeamMembershipForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::SentryV1alpha1TeamMembershipForProvider,
    #[serde(rename = "memberRef", default, skip_serializing_if = "Option::is_none")]
    pub member_ref: Option<koof::reference::ResourceReference>,
    #[serde(rename = "teamRef", default, skip_serializing_if = "Option::is_none")]
    pub team_ref: Option<koof::reference::ResourceReference>,
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
    group = "sentry.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "TeamMembership",
    namespaced,
    status = "TeamMembershipStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.sentry\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct TeamMembershipSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: TeamMembershipForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type TeamMembershipStatus = koof::status::ManagedStatus<
    crate::generated::types::SentryV1alpha1TeamMembershipAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<TeamMembership>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "TeamMembership",
        api_resource,
        "sentry.kube.nas1k.dev",
        true,
        koof::resource::ObserveDescriptor::new(
                koof::resource::OperationDescriptor::new()
                    .with_path_parameters(
                        &[
                            (
                                "organization_id_or_slug",
                                koof::resource::ValueExpr::RelatedIdentifier {
                                    via: &["team", "organization"],
                                    name: "external_id",
                                },
                            ),
                            (
                                "team_id_or_slug",
                                koof::resource::ValueExpr::RelatedIdentifier {
                                    via: &["team"],
                                    name: "external_id",
                                },
                            ),
                        ],
                    ),
            )
            .with_match(
                koof::resource::ResponseMatch::new(
                    "/id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["member"],
                        name: "external_id",
                    },
                ),
            ),
    )
    .with_identifiers(&[("external_id", koof::resource::ValueExpr::ExternalName)])
    .with_late_initialize(&["teamRole"])
    .with_references(
        &[
            koof::resource::Reference::new(
                "member",
                "/spec/forProvider/memberRef",
                &[&crate::generated::sentry::v1alpha1::organization_member::DESCRIPTOR],
            ),
            koof::resource::Reference::new(
                "team",
                "/spec/forProvider/teamRef",
                &[&crate::generated::sentry::v1alpha1::team::DESCRIPTOR],
            ),
        ],
    )
    .with_credentials(
        koof::resource::Credentials::new(
            koof::resource::CredentialSource::Related {
                via: &["team", "organization"],
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
                            "member_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["member"],
                                name: "external_id",
                            },
                        ),
                        (
                            "organization_id_or_slug",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["team", "organization"],
                                name: "external_id",
                            },
                        ),
                        (
                            "team_id_or_slug",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["team"],
                                name: "external_id",
                            },
                        ),
                    ],
                ),
            koof::resource::RequestMapping::new("/spec/forProvider"),
            "/id",
        ),
    )
    .with_update(
        koof::resource::UpdateDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        (
                            "member_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["member"],
                                name: "external_id",
                            },
                        ),
                        (
                            "organization_id_or_slug",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["team", "organization"],
                                name: "external_id",
                            },
                        ),
                        (
                            "team_id_or_slug",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["team"],
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
                    (
                        "member_id",
                        koof::resource::ValueExpr::RelatedIdentifier {
                            via: &["member"],
                            name: "external_id",
                        },
                    ),
                    (
                        "organization_id_or_slug",
                        koof::resource::ValueExpr::RelatedIdentifier {
                            via: &["team", "organization"],
                            name: "external_id",
                        },
                    ),
                    (
                        "team_id_or_slug",
                        koof::resource::ValueExpr::RelatedIdentifier {
                            via: &["team"],
                            name: "external_id",
                        },
                    ),
                ],
            ),
    );
impl koof::ManagedResource for TeamMembership {
    type AtProvider = crate::generated::types::SentryV1alpha1TeamMembershipAtProvider;
    type Observe = crate::generated::operations::TeamMembershipObserve;
    type Create = crate::generated::operations::TeamMembershipCreate;
    type Update = crate::generated::operations::TeamMembershipUpdate;
    type Delete = crate::generated::operations::TeamMembershipDelete;
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
