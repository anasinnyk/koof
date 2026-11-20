//! This file is generated. Do not edit manually.
#![allow(clippy::pedantic)]
/// CRD schema of `spec.forProvider`: unions flattened for kube, then the
/// overlay's CEL validations.
fn for_provider_schema(schema: &mut schemars::Schema) {
    koof::schema::flatten_unions(schema);
    koof::schema::add_list_maps(
        schema,
        &[
            koof::schema::ListMap {
                field: "files",
                keys: &["name"],
            },
        ],
    );
    koof::schema::add_validations(
        schema,
        &[
            koof::schema::Validation {
                rule: "self == oldSelf",
                message: "snippet_name is immutable",
                field: "snippet_name",
            },
            koof::schema::Validation {
                rule: "!has(oldSelf.snippet_name) || has(self.snippet_name)",
                message: "snippet_name cannot be removed once set",
                field: "",
            },
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
pub struct SnippetForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::ZonesV1alpha1SnippetForProvider,
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
    kind = "Snippet",
    namespaced,
    status = "SnippetStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.zones\\\\.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct SnippetSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: SnippetForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type SnippetStatus = koof::status::ManagedStatus<
    crate::generated::types::ZonesV1alpha1SnippetAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<Snippet>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "Snippet",
        api_resource,
        "zones.cloudflare.kube.nas1k.dev",
        true,
        koof::resource::ObserveDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        ("snippet_name", koof::resource::ValueExpr::ExternalName),
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
    .with_late_initialize(&["files", "main_module", "snippet_name"])
    .with_list_map_keys(&[("files", &["name"])])
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
    .with_create(
        koof::resource::CreateDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        (
                            "snippet_name",
                            koof::resource::ValueExpr::Coalesce(
                                &[
                                    koof::resource::ValueExpr::ExternalName,
                                    koof::resource::ValueExpr::Field(
                                        "/spec/forProvider/snippet_name",
                                    ),
                                ],
                            ),
                        ),
                        (
                            "zone_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["zone"],
                                name: "external_id",
                            },
                        ),
                    ],
                ),
            koof::resource::RequestMapping::new("/spec/forProvider")
                .with_set(
                    &[
                        (
                            "/metadata/main_module",
                            koof::resource::ValueExpr::Field(
                                "/spec/forProvider/main_module",
                            ),
                        ),
                    ],
                ),
            "/result/snippet_name",
        ),
    )
    .with_update(
        koof::resource::UpdateDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        (
                            "snippet_name",
                            koof::resource::ValueExpr::Coalesce(
                                &[
                                    koof::resource::ValueExpr::ExternalName,
                                    koof::resource::ValueExpr::Field(
                                        "/spec/forProvider/snippet_name",
                                    ),
                                ],
                            ),
                        ),
                        (
                            "zone_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["zone"],
                                name: "external_id",
                            },
                        ),
                    ],
                ),
            koof::resource::RequestMapping::new("/spec/forProvider")
                .with_set(
                    &[
                        (
                            "/metadata/main_module",
                            koof::resource::ValueExpr::Field(
                                "/spec/forProvider/main_module",
                            ),
                        ),
                    ],
                ),
        ),
    )
    .with_delete(
        koof::resource::OperationDescriptor::new()
            .with_path_parameters(
                &[
                    ("snippet_name", koof::resource::ValueExpr::ExternalName),
                    (
                        "zone_id",
                        koof::resource::ValueExpr::RelatedIdentifier {
                            via: &["zone"],
                            name: "external_id",
                        },
                    ),
                ],
            ),
    )
    .with_merge(
        koof::resource::MergeDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        ("snippet_name", koof::resource::ValueExpr::ExternalName),
                        (
                            "zone_id",
                            koof::resource::ValueExpr::RelatedIdentifier {
                                via: &["zone"],
                                name: "external_id",
                            },
                        ),
                    ],
                ),
            "files",
        ),
    );
impl koof::ManagedResource for Snippet {
    type AtProvider = crate::generated::types::ZonesV1alpha1SnippetAtProvider;
    type Observe = crate::generated::operations::SnippetObserve;
    type Create = crate::generated::operations::SnippetCreate;
    type Update = crate::generated::operations::SnippetUpdate;
    type Delete = crate::generated::operations::SnippetDelete;
    type Connection = koof::http::Unsupported;
    type Merge = crate::generated::operations::SnippetMerge;
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
