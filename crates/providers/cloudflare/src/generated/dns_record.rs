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
pub struct DNSRecordForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::DnsRecordForProvider,
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
    kind = "DNSRecord",
    namespaced,
    status = "DNSRecordStatus"
)]
pub struct DNSRecordSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: DNSRecordForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type DNSRecordStatus = koof::status::ManagedStatus<
    crate::generated::types::DnsRecordAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<DNSRecord>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "DNSRecord",
    api_resource,
    group: "zones.cloudflare.kube.nas1k.dev",
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    references: &[
        koof::resource::Reference {
            name: "zone",
            from: "spec.forProvider.zoneRef",
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
    observe: koof::resource::OperationDescriptor {
        path_parameters: &[
            ("dns_record_id", koof::resource::ValueExpr::ExternalName),
            (
                "zone_id",
                koof::resource::ValueExpr::RelatedIdentifier {
                    via: &["zone"],
                    name: "external_id",
                },
            ),
        ],
        response_path: "result",
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
            from: "spec.forProvider",
            set: &[],
        },
        external_name: "result.id",
    }),
    update: Some(koof::resource::UpdateDescriptor {
        operation: koof::resource::OperationDescriptor {
            path_parameters: &[
                ("dns_record_id", koof::resource::ValueExpr::ExternalName),
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
            from: "spec.forProvider",
            set: &[],
        },
    }),
    delete: Some(koof::resource::OperationDescriptor {
        path_parameters: &[
            ("dns_record_id", koof::resource::ValueExpr::ExternalName),
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
impl koof::ManagedResource for DNSRecord {
    type AtProvider = crate::generated::types::DnsRecordAtProvider;
    type Observe = crate::generated::operations::DnsRecordObserve;
    type Create = crate::generated::operations::DnsRecordCreate;
    type Update = crate::generated::operations::DnsRecordUpdate;
    type Delete = crate::generated::operations::DnsRecordDelete;
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
