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
                message: "name is immutable",
                field: "name",
            },
            koof::schema::Validation {
                rule: "!has(oldSelf.name) || has(self.name)",
                message: "name cannot be removed once set",
                field: "",
            },
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
pub struct PagesDomainForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::CloudflareV1alpha1PagesDomainForProvider,
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
    group = "cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "PagesDomain",
    namespaced,
    status = "PagesDomainStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct PagesDomainSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: PagesDomainForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type PagesDomainStatus = koof::status::ManagedStatus<
    crate::generated::types::CloudflareV1alpha1PagesDomainAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<PagesDomain>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "PagesDomain",
        api_resource,
        "cloudflare.kube.nas1k.dev",
        true,
        koof::resource::ObserveDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        (
                            "account_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["project", "account"],
                                name: "external_id",
                            },
                        ),
                        (
                            "domain_name",
                            koof::resource::ValueExpr::Coalesce(
                                &[
                                    koof::resource::ValueExpr::ExternalName,
                                    koof::resource::ValueExpr::Field("/spec/forProvider/name"),
                                ],
                            ),
                        ),
                        (
                            "project_name",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["project"],
                                name: "external_id",
                            },
                        ),
                    ],
                )
                .with_response_path("/result"),
        ),
    )
    .with_identifiers(&[("external_id", koof::resource::ValueExpr::ExternalName)])
    .with_late_initialize(&["name"])
    .with_references(
        &[
            koof::resource::Reference::new(
                "project",
                "/spec/forProvider/projectRef",
                &[&crate::generated::cloudflare::v1alpha1::pages_project::DESCRIPTOR],
            ),
        ],
    )
    .with_credentials(
        koof::resource::Credentials::new(
            koof::resource::CredentialSource::Related {
                via: &["project", "account"],
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
                            "account_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["project", "account"],
                                name: "external_id",
                            },
                        ),
                        (
                            "project_name",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["project"],
                                name: "external_id",
                            },
                        ),
                    ],
                ),
            koof::resource::RequestMapping::new("/spec/forProvider"),
            "/result/name",
        ),
    )
    .with_delete(
        koof::resource::OperationDescriptor::new()
            .with_path_parameters(
                &[
                    (
                        "account_id",
                        koof::resource::ValueExpr::RelatedIdentifier {
                            via: &["project", "account"],
                            name: "external_id",
                        },
                    ),
                    ("domain_name", koof::resource::ValueExpr::ExternalName),
                    (
                        "project_name",
                        koof::resource::ValueExpr::RelatedIdentifier {
                            via: &["project"],
                            name: "external_id",
                        },
                    ),
                ],
            ),
    );
impl koof::ManagedResource for PagesDomain {
    type AtProvider = crate::generated::types::CloudflareV1alpha1PagesDomainAtProvider;
    type Observe = crate::generated::operations::PagesDomainObserve;
    type Create = crate::generated::operations::PagesDomainCreate;
    type Update = koof::http::Unsupported;
    type Delete = crate::generated::operations::PagesDomainDelete;
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
