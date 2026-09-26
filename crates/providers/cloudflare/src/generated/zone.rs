//! This file is generated. Do not edit manually.
#![allow(clippy::pedantic)]
/// CRD schema of `spec.forProvider`: unions flattened for kube, then the
/// overlay's CEL validations.
fn for_provider_schema(schema: &mut schemars::Schema) {
    koof::schema::flatten_object_unions(schema);
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
pub struct ZoneForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::ZoneForProvider,
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
    kind = "Zone",
    namespaced,
    status = "ZoneStatus"
)]
pub struct ZoneSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: ZoneForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type ZoneStatus = koof::status::ManagedStatus<
    crate::generated::types::ZoneAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<Zone>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "Zone",
    api_resource,
    group: "cloudflare.kube.nas1k.dev",
    namespaced: true,
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    late_initialize: &["name", "paused", "type", "vanity_name_servers"],
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
        path_parameters: &[("zone_id", koof::resource::ValueExpr::ExternalName)],
        response_path: "result",
    },
    create: Some(koof::resource::CreateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[],
            response_path: "",
        },
        request: koof::resource::RequestMapping {
            from: "spec.forProvider",
            set: &[
                (
                    "account.id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["account"],
                        name: "external_id",
                    },
                ),
            ],
        },
        external_name: "result.id",
    }),
    update: Some(koof::resource::UpdateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[("zone_id", koof::resource::ValueExpr::ExternalName)],
            response_path: "",
        },
        request: koof::resource::RequestMapping {
            from: "spec.forProvider",
            set: &[],
        },
    }),
    delete: Some(koof::resource::OperationDescriptor {
        path_parameters: &[("zone_id", koof::resource::ValueExpr::ExternalName)],
        response_path: "",
    }),
};
impl koof::ManagedResource for Zone {
    type AtProvider = crate::generated::types::ZoneAtProvider;
    type Observe = crate::generated::operations::ZoneObserve;
    type Create = crate::generated::operations::ZoneCreate;
    type Update = crate::generated::operations::ZoneUpdate;
    type Delete = crate::generated::operations::ZoneDelete;
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
