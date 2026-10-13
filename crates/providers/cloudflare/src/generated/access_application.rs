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
pub struct AccessApplicationForProviderSpec {
    #[serde(flatten)]
    pub fields: crate::generated::types::AccessApplicationForProvider,
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
    group = "zerotrust.cloudflare.kube.nas1k.dev",
    version = "v1alpha1",
    kind = "AccessApplication",
    namespaced,
    status = "AccessApplicationStatus",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Ready\\\")].status\",\"name\":\"Ready\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".status.conditions[?(@.type==\\\"Synced\\\")].status\",\"name\":\"Synced\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.annotations.zerotrust\\\\.cloudflare\\\\.kube\\\\.nas1k\\\\.dev/external-name\",\"name\":\"External-Name\",\"type\":\"string\"}",
    printcolumn = "{\"jsonPath\":\".metadata.creationTimestamp\",\"name\":\"Age\",\"type\":\"date\"}"
)]
pub struct AccessApplicationSpec {
    #[serde(rename = "forProvider")]
    pub for_provider: AccessApplicationForProviderSpec,
    #[serde(flatten)]
    pub management: koof::managed::ManagedResourceSpec,
}
pub type AccessApplicationStatus = koof::status::ManagedStatus<
    crate::generated::types::AccessApplicationAtProvider,
>;
fn api_resource() -> kube::api::ApiResource {
    kube::api::ApiResource::erase::<AccessApplication>(&())
}
pub static DESCRIPTOR: koof::ResourceDescriptor = koof::ResourceDescriptor {
    kind: "AccessApplication",
    api_resource,
    group: "zerotrust.cloudflare.kube.nas1k.dev",
    namespaced: true,
    identifiers: &[("external_id", koof::resource::ValueExpr::ExternalName)],
    late_initialize: &[
        "allow_authenticate_via_warp",
        "allow_iframe",
        "allowed_idps",
        "app_launcher_logo_url",
        "app_launcher_visible",
        "auto_redirect_to_identity",
        "bg_color",
        "cors_headers",
        "custom_deny_message",
        "custom_deny_url",
        "custom_non_identity_deny_url",
        "custom_pages",
        "destinations",
        "domain",
        "eager_redirect_cookie_setting",
        "enable_binding_cookie",
        "footer_links",
        "header_bg_color",
        "http_only_cookie_attribute",
        "landing_page_design",
        "logo_url",
        "mfa_config",
        "name",
        "oauth_configuration",
        "options_preflight_bypass",
        "path_cookie_attribute",
        "policies",
        "read_service_tokens_from_header",
        "saas_app",
        "same_site_cookie_attribute",
        "scim_config",
        "self_hosted_domains",
        "service_auth_401_redirect",
        "session_duration",
        "skip_app_launcher_login_page",
        "skip_interstitial",
        "tags",
        "target_criteria",
        "type",
        "use_clientless_isolation_app_launcher_url",
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
                ("app_id", koof::resource::ValueExpr::ExternalName),
            ],
            response_path: "result",
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
            from: "spec.forProvider",
            set: &[],
        },
        external_name: "result.id",
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
                ("app_id", koof::resource::ValueExpr::ExternalName),
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
            ("app_id", koof::resource::ValueExpr::ExternalName),
        ],
        response_path: "",
    }),
};
impl koof::ManagedResource for AccessApplication {
    type AtProvider = crate::generated::types::AccessApplicationAtProvider;
    type Observe = crate::generated::operations::AccessApplicationObserve;
    type Create = crate::generated::operations::AccessApplicationCreate;
    type Update = crate::generated::operations::AccessApplicationUpdate;
    type Delete = crate::generated::operations::AccessApplicationDelete;
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
