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
pub struct ProjectForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::SentryV1alpha1ProjectForProvider,
    #[serde(
        rename = "organizationRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_ref: Option<
        crate::generated::sentry::v1alpha1::organization::OrganizationReference,
    >,
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
    kind = "Project",
    namespaced,
    status = "ProjectStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.sentry\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct ProjectSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: ProjectForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type ProjectStatus = koof::status::ManagedStatus<
    crate::generated::types::SentryV1alpha1ProjectAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<Project>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "Project",
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
                                via: &["organization"],
                                name: "external_id",
                            },
                        ),
                        ("project_id_or_slug", koof::resource::ValueExpr::ExternalName),
                    ],
                ),
        ),
    )
    .with_identifiers(&[("external_id", koof::resource::ValueExpr::ExternalName)])
    .with_late_initialize(&["default_rules", "name", "platform", "slug"])
    .with_references(
        &[
            koof::resource::Reference::new(
                "organization",
                "/spec/forProvider/organizationRef",
                &[
                    &crate::generated::sentry::v1alpha1::organization::DESCRIPTOR,
                    &crate::generated::sentry::v1alpha1::cluster_organization::DESCRIPTOR,
                ],
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
                via: &["organization"],
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
                                via: &["organization"],
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
                            "organization_id_or_slug",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["organization"],
                                name: "external_id",
                            },
                        ),
                        ("project_id_or_slug", koof::resource::ValueExpr::ExternalName),
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
                        "organization_id_or_slug",
                        koof::resource::ValueExpr::RelatedIdentifier {
                            via: &["organization"],
                            name: "external_id",
                        },
                    ),
                    ("project_id_or_slug", koof::resource::ValueExpr::ExternalName),
                ],
            ),
    );
impl koof::ManagedResource for Project {
    type AtProvider = crate::generated::types::SentryV1alpha1ProjectAtProvider;
    type Observe = crate::generated::operations::ProjectObserve;
    type Create = crate::generated::operations::ProjectCreate;
    type Update = crate::generated::operations::ProjectUpdate;
    type Delete = crate::generated::operations::ProjectDelete;
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
