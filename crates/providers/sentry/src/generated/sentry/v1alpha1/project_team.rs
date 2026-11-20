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
                message: "projectRef is immutable",
                field: "projectRef",
            },
            koof::schema::Validation {
                rule: "!has(oldSelf.projectRef) || has(self.projectRef)",
                message: "projectRef cannot be removed once set",
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
pub struct ProjectTeamForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::SentryV1alpha1ProjectTeamForProvider,
    #[serde(rename = "projectRef", default, skip_serializing_if = "Option::is_none")]
    pub project_ref: Option<koof::reference::ResourceReference>,
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
    kind = "ProjectTeam",
    namespaced,
    status = "ProjectTeamStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.sentry\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct ProjectTeamSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: ProjectTeamForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type ProjectTeamStatus = koof::status::ManagedStatus<
    crate::generated::types::SentryV1alpha1ProjectTeamAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<ProjectTeam>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "ProjectTeam",
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
                                    via: &["project", "organization"],
                                    name: "external_id",
                                },
                            ),
                            (
                                "project_id_or_slug",
                                koof::resource::ValueExpr::RelatedIdentifier {
                                    via: &["project"],
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
                        via: &["team"],
                        name: "external_id",
                    },
                ),
            ),
    )
    .with_identifiers(&[("external_id", koof::resource::ValueExpr::ExternalName)])
    .with_references(
        &[
            koof::resource::Reference::new(
                "project",
                "/spec/forProvider/projectRef",
                &[&crate::generated::sentry::v1alpha1::project::DESCRIPTOR],
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
                via: &["project", "organization"],
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
                            "organization_id_or_slug",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["project", "organization"],
                                name: "external_id",
                            },
                        ),
                        (
                            "project_id_or_slug",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["project"],
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
    .with_delete(
        koof::resource::OperationDescriptor::new()
            .with_path_parameters(
                &[
                    (
                        "organization_id_or_slug",
                        koof::resource::ValueExpr::RelatedIdentifier {
                            via: &["project", "organization"],
                            name: "external_id",
                        },
                    ),
                    (
                        "project_id_or_slug",
                        koof::resource::ValueExpr::RelatedIdentifier {
                            via: &["project"],
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
impl koof::ManagedResource for ProjectTeam {
    type AtProvider = crate::generated::types::SentryV1alpha1ProjectTeamAtProvider;
    type Observe = crate::generated::operations::ProjectTeamObserve;
    type Create = crate::generated::operations::ProjectTeamCreate;
    type Update = koof::http::Unsupported;
    type Delete = crate::generated::operations::ProjectTeamDelete;
    type Connection = koof::http::Unsupported;
    type Merge = koof::http::Unsupported;
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
