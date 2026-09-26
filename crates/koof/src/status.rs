//! Status shared by every managed resource.

use crate::{
    conditions::{self, ConditionSpec},
    error::{ReconcileError, ResolveValueError},
    reconciler::Observation,
    resource::ManagedResource,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
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

/// Outcome of a status update, for events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusChange {
    /// `Ready` switched to `True` with this update.
    pub became_ready: bool,
}

/// After a successful observe: writes `atProvider`, `observedGeneration`, `Ready`
/// and `Synced=True`; skips the patch when nothing changes.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource has no namespace, the observation
/// does not match `R::AtProvider`, or the status patch fails.
pub async fn update_status<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    observation: &Observation,
) -> Result<StatusChange, ReconcileError> {
    let at_provider: Option<R::AtProvider> = observation
        .at_provider
        .clone()
        .map(serde_json::from_value)
        .transpose()?;
    let generation = resource.meta().generation;
    let current = current_conditions(resource);

    let ready = if observation.exists {
        ConditionSpec::available()
    } else {
        ConditionSpec::unavailable("External resource does not exist")
    };
    let conditions = conditions::set(&current, ready, generation);
    let conditions = conditions::set(&conditions, ConditionSpec::reconcile_success(), generation);
    let change = StatusChange {
        became_ready: !conditions::is_true(&current, conditions::READY)
            && conditions::is_true(&conditions, conditions::READY),
    };

    let desired = ManagedStatus {
        at_provider,
        observed_generation: generation,
        conditions,
    };
    if serde_json::to_value(resource.status())? == serde_json::to_value(Some(&desired))? {
        return Ok(change);
    }
    patch_status(client, resource, &serde_json::json!({ "status": desired })).await?;
    Ok(change)
}

/// After a failed reconcile: sets `Synced=False` with the error, keeping everything
/// else in the status.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource has no namespace or the patch fails.
pub async fn record_failure<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    error: &ReconcileError,
) -> Result<(), ReconcileError> {
    let current = current_conditions(resource);
    let conditions = conditions::set(
        &current,
        ConditionSpec::reconcile_error(error.to_string()),
        resource.meta().generation,
    );
    if conditions == current {
        return Ok(());
    }
    patch_status(
        client,
        resource,
        &serde_json::json!({ "status": { "conditions": conditions } }),
    )
    .await
}

fn current_conditions<R: ManagedResource>(resource: &R) -> Vec<Condition> {
    resource
        .status()
        .map(|status| status.conditions.clone())
        .unwrap_or_default()
}

async fn patch_status<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    patch: &serde_json::Value,
) -> Result<(), ReconcileError> {
    let namespace = kube::ResourceExt::namespace(resource).ok_or_else(|| {
        ResolveValueError::Missing(format!("{} has no namespace", R::descriptor().kind))
    })?;
    kube::Api::<R>::namespaced(client.clone(), &namespace)
        .patch_status(
            &kube::ResourceExt::name_any(resource),
            &kube::api::PatchParams::default(),
            &kube::api::Patch::Merge(patch),
        )
        .await?;
    Ok(())
}
