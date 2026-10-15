use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// `status` of a managed resource; `A` is the generated `atProvider` type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ManagedStatus<A> {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(transform = crate::schema::flatten_unions)]
    pub at_provider: Option<A>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    #[serde(default)]
    pub conditions: Vec<Condition>,
}
