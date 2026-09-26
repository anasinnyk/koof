//! The managed-resource finalizer: added before the controller may change anything
//! externally, removed once the external resource is deleted or orphaned.
//!
//! Patches carry the full finalizer list together with `metadata.resourceVersion`,
//! so a concurrent change to the object makes the patch fail with a conflict (a
//! transient error, retried) instead of dropping someone else's finalizer.

use crate::{
    error::{ReconcileError, ResolveValueError},
    resource::ManagedResource,
};

/// Whether `resource` carries its finalizer.
#[must_use]
pub fn has_finalizer<R: ManagedResource>(resource: &R) -> bool {
    kube::ResourceExt::finalizers(resource)
        .iter()
        .any(|finalizer| finalizer == R::descriptor().finalizer)
}

/// Adds the finalizer unless it is already there.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch fails (including a conflict).
pub async fn ensure<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
) -> Result<(), ReconcileError> {
    if has_finalizer(resource) {
        return Ok(());
    }
    let mut finalizers = kube::ResourceExt::finalizers(resource).to_vec();
    finalizers.push(R::descriptor().finalizer.to_owned());
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
    let finalizers: Vec<String> = kube::ResourceExt::finalizers(resource)
        .iter()
        .filter(|finalizer| *finalizer != R::descriptor().finalizer)
        .cloned()
        .collect();
    patch(client, resource, &finalizers).await
}

async fn patch<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    finalizers: &[String],
) -> Result<(), ReconcileError> {
    let namespace = kube::ResourceExt::namespace(resource).ok_or_else(|| {
        ResolveValueError::Missing(format!("{} has no namespace", R::descriptor().kind))
    })?;
    let patch = serde_json::json!({
        "metadata": {
            "finalizers": finalizers,
            "resourceVersion": kube::ResourceExt::resource_version(resource),
        }
    });
    kube::Api::<R>::namespaced(client.clone(), &namespace)
        .patch(
            &kube::ResourceExt::name_any(resource),
            &kube::api::PatchParams::default(),
            &kube::api::Patch::Merge(&patch),
        )
        .await?;
    Ok(())
}
