//! Writes to managed resources regardless of scope.
//!
//! A typed `Api<R>` fixes namespaced vs cluster scope at compile time; the runtime
//! is generic over both, so it patches through `Api<DynamicObject>` with the
//! resource's `ApiResource` and converts the result back to `R`.

use crate::{error::ReconcileError, resource::ManagedResource};
use kube::api::{Api, DynamicObject, Patch, PatchParams};

fn api<R: ManagedResource>(client: &kube::Client, resource: &R) -> Api<DynamicObject> {
    let api_resource = (R::descriptor().api_resource)();
    match kube::ResourceExt::namespace(resource) {
        Some(namespace) => Api::namespaced_with(client.clone(), &namespace, &api_resource),
        None => Api::all_with(client.clone(), &api_resource),
    }
}

/// Merge-patches the object; returns it as `R`.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch fails (a `resourceVersion` in the patch
/// turns concurrent changes into conflicts) or the result does not decode as `R`.
pub async fn patch<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    patch: &serde_json::Value,
) -> Result<R, ReconcileError> {
    let object = api(client, resource)
        .patch(
            &kube::ResourceExt::name_any(resource),
            &PatchParams::default(),
            &Patch::Merge(patch),
        )
        .await?;
    Ok(serde_json::from_value(serde_json::to_value(object)?)?)
}

/// Merge-patches the status subresource.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch fails.
pub async fn patch_status<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    patch: &serde_json::Value,
) -> Result<(), ReconcileError> {
    api(client, resource)
        .patch_status(
            &kube::ResourceExt::name_any(resource),
            &PatchParams::default(),
            &Patch::Merge(patch),
        )
        .await?;
    Ok(())
}
