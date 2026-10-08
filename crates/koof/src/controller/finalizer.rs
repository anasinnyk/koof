//! The managed-resource finalizer: added before the controller may change anything
//! externally, removed once the external resource is deleted or orphaned.
//!
//! Patches carry the full finalizer list together with `metadata.resourceVersion`
//! ([`crate::object::patch_guarded`]), so a concurrent change to the object makes
//! the patch fail with a conflict instead of dropping someone else's finalizer.

use crate::{error::ReconcileError, object, resource::ManagedResource};

/// Whether `resource` carries its finalizer.
#[must_use]
pub fn has_finalizer<R: ManagedResource>(resource: &R) -> bool {
    kube::ResourceExt::finalizers(resource)
        .iter()
        .any(|finalizer| *finalizer == R::descriptor().finalizer())
}

/// Adds the finalizer unless it is already there; returns the up-to-date object
/// (use it for further patches, its `resourceVersion` changed).
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch fails (including a conflict).
pub async fn ensure<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
) -> Result<R, ReconcileError> {
    if has_finalizer(resource) {
        return Ok(resource.clone());
    }
    let mut finalizers = kube::ResourceExt::finalizers(resource).to_vec();
    finalizers.push(R::descriptor().finalizer());
    patch(client, resource, &finalizers).await
}

/// Removes the finalizer if present; afterwards Kubernetes may delete the object.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch fails (including a conflict).
pub async fn remove<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
) -> Result<(), ReconcileError> {
    if !has_finalizer(resource) {
        return Ok(());
    }
    let own = R::descriptor().finalizer();
    let finalizers: Vec<String> = kube::ResourceExt::finalizers(resource)
        .iter()
        .filter(|finalizer| **finalizer != own)
        .cloned()
        .collect();
    patch(client, resource, &finalizers).await?;
    Ok(())
}

async fn patch<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    finalizers: &[String],
) -> Result<R, ReconcileError> {
    object::patch_guarded(
        client,
        resource,
        serde_json::json!({ "metadata": { "finalizers": finalizers } }),
    )
    .await
}
