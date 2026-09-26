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
pub struct PagesProjectForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::PagesProjectForProvider,
    #[serde(rename = "accountRef", default, skip_serializing_if = "Option::is_none")]
    pub account_ref: Option<koof::reference::ResourceReference>,
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
    kind = "PagesProject",
    namespaced,
    status = "PagesProjectStatus"
)]
pub struct PagesProjectSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: PagesProjectForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type PagesProjectStatus = koof::status::ManagedStatus<
    crate::generated::types::PagesProjectAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<PagesProject>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "PagesProject",
    api_resource,
    group: "cloudflare.kube.nas1k.dev",
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    late_initialize: &[
        "build_config",
        "deployment_configs",
        "name",
        "production_branch",
        "source",
    ],
    references: &[
        koof::resource::Reference {
            name: "account",
            from: "spec.forProvider.accountRef",
            target: &crate::generated::account::DESCRIPTOR,
            namespace: koof::resource::NamespacePolicy::SameAsResource,
        },
    ],
    credentials: Some(koof::resource::Credentials {
        source: koof::resource::CredentialSource::Related {
            via: &["account"],
        },
        auth: koof::resource::AuthScheme::Bearer,
    }),
    observe: koof::resource::OperationDescriptor {
        path_parameters: &[
            (
                "account_id",
                koof::resource::ValueExpr::RelatedIdentifier {
                    via: &["account"],
                    name: "external_id",
                },
            ),
            (
                "project_name",
                koof::resource::ValueExpr::Coalesce(
                    &[
                        koof::resource::ValueExpr::ExternalName,
                        koof::resource::ValueExpr::Field("spec.resourceName"),
                        koof::resource::ValueExpr::Field("metadata.name"),
                    ],
                ),
            ),
        ],
        response_path: "result",
    },
    create: Some(koof::resource::CreateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                (
                    "account_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["account"],
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
        external_name: "result.name",
    }),
    update: Some(koof::resource::UpdateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                (
                    "account_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["account"],
                        name: "external_id",
                    },
                ),
                (
                    "project_name",
                    koof::resource::ValueExpr::Coalesce(
                        &[
                            koof::resource::ValueExpr::ExternalName,
                            koof::resource::ValueExpr::Field("spec.resourceName"),
                            koof::resource::ValueExpr::Field("metadata.name"),
                        ],
                    ),
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
            (
                "account_id",
                koof::resource::ValueExpr::RelatedIdentifier {
                    via: &["account"],
                    name: "external_id",
                },
            ),
            (
                "project_name",
                koof::resource::ValueExpr::Coalesce(
                    &[
                        koof::resource::ValueExpr::ExternalName,
                        koof::resource::ValueExpr::Field("spec.resourceName"),
                        koof::resource::ValueExpr::Field("metadata.name"),
                    ],
                ),
            ),
        ],
        response_path: "",
    }),
};
impl koof::ManagedResource for PagesProject {
    type AtProvider = crate::generated::types::PagesProjectAtProvider;
    type Observe = crate::generated::operations::PagesProjectObserve;
    type Create = crate::generated::operations::PagesProjectCreate;
    type Update = crate::generated::operations::PagesProjectUpdate;
    type Delete = crate::generated::operations::PagesProjectDelete;
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
