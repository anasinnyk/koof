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
pub struct ProjectKeyForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::SentryV1alpha1ProjectKeyForProvider,
    #[serde(rename = "projectRef", default, skip_serializing_if = "Option::is_none")]
    pub project_ref: Option<koof::reference::ResourceReference>,
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
    kind = "ProjectKey",
    namespaced,
    status = "ProjectKeyStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.sentry\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct ProjectKeySpec {
    #[serde(rename = "forProvider")]
    pub for_provider: ProjectKeyForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type ProjectKeyStatus = koof::status::ManagedStatus<
    crate::generated::types::SentryV1alpha1ProjectKeyAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<ProjectKey>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "ProjectKey",
        api_resource,
        "sentry.kube.nas1k.dev",
        true,
        koof::resource::ObserveDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        ("key_id", koof::resource::ValueExpr::ExternalName),
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
        ),
    )
    .with_identifiers(&[("external_id", koof::resource::ValueExpr::ExternalName)])
    .with_late_initialize(&["name", "rateLimit", "useCase"])
    .with_references(
        &[
            koof::resource::Reference::new(
                "project",
                "/spec/forProvider/projectRef",
                &[&crate::generated::sentry::v1alpha1::project::DESCRIPTOR],
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
                        ("key_id", koof::resource::ValueExpr::ExternalName),
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
            koof::resource::RequestMapping::new("/spec/forProvider"),
        ),
    )
    .with_delete(
        koof::resource::OperationDescriptor::new()
            .with_path_parameters(
                &[
                    ("key_id", koof::resource::ValueExpr::ExternalName),
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
    );
impl koof::ManagedResource for ProjectKey {
    type AtProvider = crate::generated::types::SentryV1alpha1ProjectKeyAtProvider;
    type Observe = crate::generated::operations::ProjectKeyObserve;
    type Create = crate::generated::operations::ProjectKeyCreate;
    type Update = crate::generated::operations::ProjectKeyUpdate;
    type Delete = crate::generated::operations::ProjectKeyDelete;
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
