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
pub struct ProjectKeyForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::ProjectKeyForProvider,
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
    status = "ProjectKeyStatus"
)]
pub struct ProjectKeySpec {
    #[serde(rename = "forProvider")]
    pub for_provider: ProjectKeyForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type ProjectKeyStatus = koof::status::ManagedStatus<
    crate::generated::types::ProjectKeyAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<ProjectKey>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "ProjectKey",
    api_resource,
    group: "sentry.kube.nas1k.dev",
    namespaced: true,
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    late_initialize: &["name", "rateLimit", "useCase"],
    references: &[
        koof::resource::Reference {
            name: "project",
            from: "spec.forProvider.projectRef",
            target: &crate::generated::project::DESCRIPTOR,
            namespace: koof::resource::NamespacePolicy::SameAsResource,
        },
    ],
    credentials: Some(koof::resource::Credentials {
        source: koof::resource::CredentialSource::Related {
            via: &["project", "organization"],
        },
        auth: koof::resource::AuthScheme::Bearer,
    }),
    observe: koof::resource::OperationDescriptor {
        path_parameters: &[
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
        response_path: "",
    },
    create: Some(koof::resource::CreateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
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
            response_path: "",
        },
        request: koof::resource::RequestMapping {
            from: "spec.forProvider",
            set: &[],
        },
    }),
    delete: Some(koof::resource::OperationDescriptor {
        path_parameters: &[
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
        response_path: "",
    }),
};
impl koof::ManagedResource for ProjectKey {
    type AtProvider = crate::generated::types::ProjectKeyAtProvider;
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
