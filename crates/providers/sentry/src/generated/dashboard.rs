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
pub struct DashboardForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::DashboardForProvider,
    #[serde(
        rename = "organizationRef",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub organization_ref: Option<koof::reference::ResourceReference>,
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
    kind = "Dashboard",
    namespaced,
    status = "DashboardStatus"
)]
pub struct DashboardSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: DashboardForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type DashboardStatus = koof::status::ManagedStatus<
    crate::generated::types::DashboardAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<Dashboard>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "Dashboard",
    api_resource,
    group: "sentry.kube.nas1k.dev",
    namespaced: true,
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    late_initialize: &[
        "end",
        "environment",
        "filters",
        "id",
        "is_favorited",
        "period",
        "permissions",
        "projects",
        "start",
        "title",
        "utc",
        "widgets",
    ],
    references: &[
        koof::resource::Reference {
            name: "organization",
            from: "spec.forProvider.organizationRef",
            target: &crate::generated::organization::DESCRIPTOR,
            namespace: koof::resource::NamespacePolicy::SameAsResource,
        },
    ],
    credentials: Some(koof::resource::Credentials {
        source: koof::resource::CredentialSource::Related {
            via: &["organization"],
        },
        auth: koof::resource::AuthScheme::Bearer,
    }),
    observe: koof::resource::OperationDescriptor {
        path_parameters: &[
            ("dashboard_id", koof::resource::ValueExpr::ExternalName),
            (
                "organization_id_or_slug",
                koof::resource::ValueExpr::RelatedIdentifier {
                    via: &["organization"],
                    name: "external_id",
                },
            ),
        ],
        response_path: "",
    },
    create: Some(koof::resource::CreateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                (
                    "organization_id_or_slug",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["organization"],
                        name: "external_id",
                    },
                ),
            ],
            response_path: "",
        },
        request: koof::resource::RequestMapping {
            from: "spec.forProvider",
            set: &[],
        },
        external_name: "id",
    }),
    update: Some(koof::resource::UpdateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                ("dashboard_id", koof::resource::ValueExpr::ExternalName),
                (
                    "organization_id_or_slug",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["organization"],
                        name: "external_id",
                    },
                ),
            ],
            response_path: "",
        },
        request: koof::resource::RequestMapping {
            from: "spec.forProvider",
            set: &[],
        },
    }),
    delete: Some(koof::resource::OperationDescriptor {
        path_parameters: &[
            ("dashboard_id", koof::resource::ValueExpr::ExternalName),
            (
                "organization_id_or_slug",
                koof::resource::ValueExpr::RelatedIdentifier {
                    via: &["organization"],
                    name: "external_id",
                },
            ),
        ],
        response_path: "",
    }),
};
impl koof::ManagedResource for Dashboard {
    type AtProvider = crate::generated::types::DashboardAtProvider;
    type Observe = crate::generated::operations::DashboardObserve;
    type Create = crate::generated::operations::DashboardCreate;
    type Update = crate::generated::operations::DashboardUpdate;
    type Delete = crate::generated::operations::DashboardDelete;
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
