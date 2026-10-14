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
pub struct CustomHostnameForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::CustomHostnameForProvider,
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
    kind = "CustomHostname",
    namespaced,
    status = "CustomHostnameStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.zones\\\\.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct CustomHostnameSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: CustomHostnameForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type CustomHostnameStatus = koof::status::ManagedStatus<
    crate::generated::types::CustomHostnameAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<CustomHostname>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "CustomHostname",
    api_resource,
    group: "zones.cloudflare.kube.nas1k.dev",
    namespaced: true,
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    late_initialize: &[
        "custom_metadata",
        "custom_origin_server",
        "custom_origin_sni",
        "hostname",
        "ssl",
    ],
    references: &[
        koof::resource::Reference {
            name: "zone",
            from: "/spec/forProvider/zoneRef",
            target: &crate::generated::zone::DESCRIPTOR,
            namespace: koof::resource::NamespacePolicy::SameAsResource,
        },
    ],
    credentials: Some(koof::resource::Credentials {
        source: koof::resource::CredentialSource::Related {
            via: &["zone", "account"],
        },
        auth: koof::resource::AuthScheme::Bearer,
    }),
    observe: koof::resource::ObserveDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                ("custom_hostname_id", koof::resource::ValueExpr::ExternalName),
                (
                    "zone_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["zone"],
                        name: "external_id",
                    },
                ),
            ],
            response_path: "/result",
        },
        deleted_when: None,
    },
    create: Some(koof::resource::CreateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                (
                    "zone_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["zone"],
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
        external_name: "/result/id",
    }),
    update: Some(koof::resource::UpdateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                ("custom_hostname_id", koof::resource::ValueExpr::ExternalName),
                (
                    "zone_id",
                    koof::resource::ValueExpr::RelatedIdentifier {
                        via: &["zone"],
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
    }),
    delete: Some(koof::resource::OperationDescriptor {
        path_parameters: &[
            ("custom_hostname_id", koof::resource::ValueExpr::ExternalName),
            (
                "zone_id",
                koof::resource::ValueExpr::RelatedIdentifier {
                    via: &["zone"],
                    name: "external_id",
                },
            ),
        ],
        response_path: "",
    }),
};
impl koof::ManagedResource for CustomHostname {
    type AtProvider = crate::generated::types::CustomHostnameAtProvider;
    type Observe = crate::generated::operations::CustomHostnameObserve;
    type Create = crate::generated::operations::CustomHostnameCreate;
    type Update = crate::generated::operations::CustomHostnameUpdate;
    type Delete = crate::generated::operations::CustomHostnameDelete;
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
