//! CRDs whose `forProvider` is a typify type (no JsonSchema on it) and whose
//! schema comes straight from the synthesized OpenAPI schema.

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const CRD_SCHEMAS: &str = include_str!("crd-schemas.json");

fn crd_schema(name: &str) -> schemars::Schema {
    let all: serde_json::Value = serde_json::from_str(CRD_SCHEMAS).unwrap_or_default();
    let schema = all.get(name).cloned().unwrap_or_default();
    schemars::Schema::try_from(schema).unwrap_or_default()
}

macro_rules! crd {
    ($spec:ident, $kind:literal, $group:literal, $for_provider:ty, $schema_fn:ident, $schema_path:literal) => {
        fn $schema_fn(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
            crd_schema(concat!($kind, "ForProvider"))
        }

        #[derive(CustomResource, Clone, Debug, Serialize, Deserialize, JsonSchema)]
        #[kube(group = $group, version = "v1alpha1", kind = $kind, namespaced)]
        pub struct $spec {
            #[serde(rename = "forProvider")]
            #[schemars(schema_with = $schema_path)]
            pub for_provider: $for_provider,
        }
    };
}

crd!(AccountSpec, "Account", "cloudflare.kube.nas1k.dev",
    crate::types::AccountForProvider, account_schema, "account_schema");
crd!(ZoneSpec, "Zone", "cloudflare.kube.nas1k.dev",
    crate::types::ZoneForProvider, zone_schema, "zone_schema");
crd!(DnsRecordSpec, "DNSRecord", "zones.cloudflare.kube.nas1k.dev",
    crate::types::DnsRecordForProvider, dns_record_schema, "dns_record_schema");
crd!(RulesetSpec, "Ruleset", "rulesets.cloudflare.kube.nas1k.dev",
    crate::types::RulesetForProvider, ruleset_schema, "ruleset_schema");
crd!(PagesProjectSpec, "PagesProject", "cloudflare.kube.nas1k.dev",
    crate::types::PagesProjectForProvider, pages_project_schema, "pages_project_schema");
