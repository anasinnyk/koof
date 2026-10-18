use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A reference to another managed resource: a namespaced resource in the namespace
/// of the resource holding the reference, or a cluster-scoped one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResourceReference {
    pub name: String,
}

/// A key of a Secret in the namespace of the (namespaced) resource holding the
/// reference, for credentials.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SecretKeySelector {
    pub name: String,
    pub key: String,
}

/// A key of a Secret in `namespace`, for credentials of a cluster-scoped resource.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ClusterSecretKeySelector {
    pub name: String,
    pub key: String,
    pub namespace: String,
}
