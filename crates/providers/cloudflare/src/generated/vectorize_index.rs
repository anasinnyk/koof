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
pub struct VectorizeIndexForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::VectorizeIndexForProvider,
    #[serde(rename = "accountRef", default, skip_serializing_if = "Option::is_none")]
    pub account_ref: Option<crate::generated::account::AccountReference>,
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
    group = "storage.cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "VectorizeIndex",
    namespaced,
    status = "VectorizeIndexStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.storage\\\\.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct VectorizeIndexSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: VectorizeIndexForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type VectorizeIndexStatus = koof::status::ManagedStatus<
    crate::generated::types::VectorizeIndexAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<VectorizeIndex>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "VectorizeIndex",
    api_resource,
    group: "storage.cloudflare.kube.nas1k.dev",
    namespaced: true,
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    late_initialize: &["config", "description", "name"],
    references: &[
        koof::resource::Reference {
            name: "account",
            from: "/spec/forProvider/accountRef",
            targets: &[
                &crate::generated::account::DESCRIPTOR,
                &crate::generated::cluster_account::DESCRIPTOR,
            ],
        },
    ],
    credentials: Some(koof::resource::Credentials {
        source: koof::resource::CredentialSource::Related {
            via: &["account"],
        },
        auth: koof::resource::AuthScheme::Bearer,
    }),
    observe: koof::resource::ObserveDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                (
                    "account_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["account"],
                        name: "external_id",
                    },
                ),
                ("index_name", koof::resource::ValueExpr::ExternalName),
            ],
            response_path: "/result",
        },
        deleted_when: None,
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
            from: "/spec/forProvider",
            set: &[],
        },
        external_name: "/result/name",
    }),
    update: None,
    delete: Some(koof::resource::OperationDescriptor {
        path_parameters: &[
            (
                "account_id",
                koof::resource::ValueExpr::RelatedIdentifier {
                    via: &["account"],
                    name: "external_id",
                },
            ),
            ("index_name", koof::resource::ValueExpr::ExternalName),
        ],
        response_path: "",
    }),
};
impl koof::ManagedResource for VectorizeIndex {
    type AtProvider = crate::generated::types::VectorizeIndexAtProvider;
    type Observe = crate::generated::operations::VectorizeIndexObserve;
    type Create = crate::generated::operations::VectorizeIndexCreate;
    type Update = koof::http::Unsupported;
    type Delete = crate::generated::operations::VectorizeIndexDelete;
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
