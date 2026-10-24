use super::{
    ControllerContext,
    conditions::{self, ConditionSpec},
    object_key,
};
use crate::{
    error::ReconcileError, external::Observation, metrics::ObjectState, resource::ManagedResource,
    status::ManagedStatus,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
use kube::api::Patch;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusChange {
    pub became_ready: bool,
}

/// After a successful observe: writes `atProvider`, `observedGeneration`, `Ready`
/// and `Synced=True`; skips the patch when nothing changes.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the observation does not match
/// `R::AtProvider` or the status patch fails.
pub async fn update_status<R: ManagedResource>(
    context: &ControllerContext,
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

    record_state(context, resource, &conditions);
    let desired = ManagedStatus {
        at_provider,
        observed_generation: generation,
        conditions,
    };
    if serde_json::to_value(resource.status())? == serde_json::to_value(Some(&desired))? {
        return Ok(change);
    }
    let replace = serde_json::from_value(serde_json::json!([
        { "op": "add", "path": "/status", "value": desired },
    ]))?;
    patch_status(context, resource, &Patch::Json(replace)).await?;
    Ok(change)
}

/// After a failed reconcile: sets `Synced=False` with the error, keeping everything
/// else in the status.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch fails.
pub async fn record_failure<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
    error: &ReconcileError,
) -> Result<(), ReconcileError> {
    set_conditions(
        context,
        resource,
        vec![ConditionSpec::reconcile_error(crate::error::chain(error))],
    )
    .await
}

/// Sets the given conditions, keeping everything else in the status.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch fails.
pub async fn set_conditions<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
    specs: Vec<ConditionSpec>,
) -> Result<(), ReconcileError> {
    let current = current_conditions(resource);
    let generation = resource.meta().generation;
    let conditions = specs.into_iter().fold(current.clone(), |conditions, spec| {
        conditions::set(&conditions, spec, generation)
    });
    record_state(context, resource, &conditions);
    if conditions == current {
        return Ok(());
    }
    patch_status(
        context,
        resource,
        &Patch::Merge(serde_json::json!({ "status": { "conditions": conditions } })),
    )
    .await
}

fn current_conditions<R: ManagedResource>(resource: &R) -> Vec<Condition> {
    resource
        .status()
        .map(|status| status.conditions.clone())
        .unwrap_or_default()
}

fn record_state<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
    conditions: &[Condition],
) {
    context.metrics.object_state(
        &object_key(resource),
        R::descriptor().group_kind(),
        Some(ObjectState {
            ready: conditions::is_true(conditions, conditions::READY),
            synced: conditions::is_true(conditions, conditions::SYNCED),
        }),
    );
}

async fn patch_status<R: ManagedResource>(
    context: &ControllerContext,
    resource: &R,
    patch: &Patch<serde_json::Value>,
) -> Result<(), ReconcileError> {
    crate::object::patch_status(&context.kube_client, resource, patch).await
}
