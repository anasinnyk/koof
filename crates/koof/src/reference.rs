//! Reference fields of generated `spec.forProvider` types.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A reference to another managed resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResourceReference {
    pub name: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// A key of a Secret, for credentials. `namespace` defaults to the namespace of the
/// resource holding the reference; cluster-scoped resources must set it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SecretReference {
    pub name: String,
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
