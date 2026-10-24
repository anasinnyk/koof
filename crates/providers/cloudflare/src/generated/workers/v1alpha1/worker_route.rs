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
pub struct WorkerRouteForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::WorkersV1alpha1WorkerRouteForProvider,
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
    group = "workers.cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "WorkerRoute",
    namespaced,
    status = "WorkerRouteStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.workers\\\\.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct WorkerRouteSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: WorkerRouteForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type WorkerRouteStatus = koof::status::ManagedStatus<
    crate::generated::types::WorkersV1alpha1WorkerRouteAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<WorkerRoute>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor::new(
        "WorkerRoute",
        api_resource,
        "workers.cloudflare.kube.nas1k.dev",
        true,
        koof::resource::ObserveDescriptor::new(
            koof::resource::OperationDescriptor::new()
                .with_path_parameters(
                    &[
                        ("route_id", koof::resource::ValueExpr::ExternalName),
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
    .with_late_initialize(&["pattern", "script"])
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
                        ("route_id", koof::resource::ValueExpr::ExternalName),
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
                    ("route_id", koof::resource::ValueExpr::ExternalName),
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
impl koof::ManagedResource for WorkerRoute {
    type AtProvider = crate::generated::types::WorkersV1alpha1WorkerRouteAtProvider;
    type Observe = crate::generated::operations::WorkerRouteObserve;
    type Create = crate::generated::operations::WorkerRouteCreate;
    type Update = crate::generated::operations::WorkerRouteUpdate;
    type Delete = crate::generated::operations::WorkerRouteDelete;
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
