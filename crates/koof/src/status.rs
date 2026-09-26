//! Status shared by every managed resource.

use crate::{error::ReconcileError, reconciler::Observation, resource::ManagedResource};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{Condition, Time};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// `status` of a managed resource; `A` is the generated `atProvider` type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ManagedStatus<A> {
    #[serde(
        rename = "atProvider",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    #[schemars(transform = crate::schema::flatten_object_unions)]
    pub at_provider: Option<A>,

    #[serde(
        rename = "observedGeneration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub observed_generation: Option<i64>,

    #[serde(default)]
    pub conditions: Vec<Condition>,
}

impl<A> Default for ManagedStatus<A> {
    fn default() -> Self {
        Self {
            at_provider: None,
            observed_generation: None,
            conditions: Vec::new(),
        }
    }
}

/// Writes `atProvider`, `observedGeneration` and the `Ready` condition; skips the
/// patch when the status would not change.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource has no namespace, the observation
/// does not match `R::AtProvider`, or the status patch fails.
pub async fn update_status<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    observation: &Observation,
) -> Result<(), ReconcileError> {
    let namespace = kube::ResourceExt::namespace(resource).ok_or_else(|| {
        crate::error::ResolveValueError::Missing(format!(
            "{} has no namespace",
            R::descriptor().kind
        ))
    })?;
    let name = kube::ResourceExt::name_any(resource);

    let at_provider: Option<R::AtProvider> = observation
        .at_provider
        .clone()
        .map(serde_json::from_value)
        .transpose()?;

    let (status, reason, message) = if observation.exists {
        ("True", "Available", "External resource exists")
    } else {
        ("False", "NotFound", "External resource does not exist")
    };
    let generation = resource.meta().generation;

    let last_transition_time = resource
        .status()
        .and_then(|current| {
            current
                .conditions
                .iter()
                .find(|condition| condition.type_ == "Ready")
        })
        .filter(|condition| condition.status == status && condition.reason == reason)
        .map_or_else(
            || Time::from(jiff::Timestamp::now()),
            |condition| condition.last_transition_time.clone(),
        );

    let desired = ManagedStatus {
        at_provider,
        observed_generation: generation,
        conditions: vec![Condition {
            type_: "Ready".into(),
            status: status.into(),
            reason: reason.into(),
            message: message.into(),
            observed_generation: generation,
            last_transition_time,
        }],
    };

    if serde_json::to_value(resource.status())? == serde_json::to_value(Some(&desired))? {
        return Ok(());
    }

    kube::Api::<R>::namespaced(client.clone(), &namespace)
        .patch_status(
            &name,
            &kube::api::PatchParams::default(),
            &kube::api::Patch::Merge(&serde_json::json!({ "status": desired })),
        )
        .await?;
    Ok(())
}
