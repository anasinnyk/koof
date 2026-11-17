use crate::{
    error::ReconcileError,
    object,
    resource::{EXTERNAL_NAME, ManagedResource},
    retry::{self, RetryPolicy},
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ManagedFieldsEntry;
use std::time::Duration;

const APPLY_RETRY: RetryPolicy = RetryPolicy {
    base: Duration::from_millis(200),
    max: Duration::from_secs(2),
    attempts: Some(5),
    waiting: None,
    permanent: None,
};

/// Whether `manager` co-owns the external name annotation through server-side
/// apply, so it stays on the object when another manager stops applying it.
#[must_use]
pub fn owned<R: ManagedResource>(resource: &R, manager: &str) -> bool {
    let key = format!("f:{}", R::descriptor().annotation(EXTERNAL_NAME));
    resource
        .meta()
        .managed_fields
        .iter()
        .flatten()
        .filter(|entry| applied_by(entry, manager))
        .any(|entry| {
            entry
                .fields_v1
                .as_ref()
                .and_then(|fields| fields.0.pointer("/f:metadata/f:annotations"))
                .and_then(|annotations| annotations.get(&key))
                .is_some()
        })
}

fn applied_by(entry: &ManagedFieldsEntry, manager: &str) -> bool {
    entry.manager.as_deref() == Some(manager)
        && entry.operation.as_deref() == Some("Apply")
        && entry.subresource.as_deref().unwrap_or_default().is_empty()
}

/// Takes co-ownership of the current external name under `manager`, unless it
/// already has it; returns the updated object when it applied. A conflict (another
/// manager changed the value since the object was read) is left to the next
/// reconcile.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the apply fails for another reason.
pub async fn own<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    manager: &str,
) -> Result<Option<R>, ReconcileError> {
    let annotation = R::descriptor().annotation(EXTERNAL_NAME);
    let Some(name) = kube::ResourceExt::annotations(resource).get(&annotation) else {
        return Ok(None);
    };
    if owned(resource, manager) {
        return Ok(None);
    }
    match object::apply_annotation(client, resource, manager, &annotation, name).await {
        Ok(updated) => Ok(Some(updated)),
        Err(ReconcileError::Kubernetes(kube::Error::Api(response))) if response.code == 409 => {
            tracing::debug!(
                reason = response.message,
                "external name changed by another manager; not taking ownership"
            );
            Ok(None)
        }
        Err(error) => Err(error),
    }
}

/// Sets the external name of a created resource under `manager`, retried on
/// transient errors.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the apply still fails after the retries.
pub async fn record<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    manager: &str,
    name: &str,
) -> Result<(), ReconcileError> {
    let annotation = R::descriptor().annotation(EXTERNAL_NAME);
    retry::with_retries(&APPLY_RETRY, || {
        object::apply_annotation(client, resource, manager, &annotation, name)
    })
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests;
