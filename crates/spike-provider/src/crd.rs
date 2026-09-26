//! CRDs whose `forProvider` is a typify type; the schema is derived by kube via
//! schemars from the typify types themselves (no hand-made CRD schema).

use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

macro_rules! crd {
    ($spec:ident, $kind:literal, $group:literal, $for_provider:ty) => {
        #[derive(CustomResource, Clone, Debug, Serialize, Deserialize, JsonSchema)]
        #[kube(group = $group, version = "v1alpha1", kind = $kind, namespaced)]
        pub struct $spec {
            #[serde(rename = "forProvider")]
            #[schemars(transform = crate::schema::flatten_object_unions)]
            pub for_provider: $for_provider,
        }
    };
}

crd!(
    AccountSpec,
    "Account",
    "cloudflare.kube.nas1k.dev",
    crate::types::AccountForProvider
);
crd!(
    ZoneSpec,
    "Zone",
    "cloudflare.kube.nas1k.dev",
    crate::types::ZoneForProvider
);
crd!(
    DnsRecordSpec,
    "DNSRecord",
    "zones.cloudflare.kube.nas1k.dev",
    crate::types::DnsRecordForProvider
);
crd!(
    RulesetSpec,
    "Ruleset",
    "rulesets.cloudflare.kube.nas1k.dev",
    crate::types::RulesetForProvider
);
crd!(
    PagesProjectSpec,
    "PagesProject",
    "cloudflare.kube.nas1k.dev",
    crate::types::PagesProjectForProvider
);
