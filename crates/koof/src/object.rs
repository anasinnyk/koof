use crate::{error::ReconcileError, resource::ManagedResource};
use kube::api::{Api, DynamicObject, Patch, PatchParams};
use serde_json::Value;

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
/// Returns [`ReconcileError`] when the patch fails or the result does not decode
/// as `R`.
pub async fn patch<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    patch: &Value,
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

/// [`patch`] with the object's `metadata.resourceVersion` added: a concurrent
/// change makes it fail with a conflict (a transient error, retried) instead of
/// overwriting someone else's change. Returns the up-to-date object (use it for
/// further patches, its `resourceVersion` changed).
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch fails (including a conflict) or the
/// result does not decode as `R`.
pub async fn patch_guarded<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    mut changes: Value,
) -> Result<R, ReconcileError> {
    changes["metadata"]["resourceVersion"] =
        serde_json::json!(kube::ResourceExt::resource_version(resource));
    patch(client, resource, &changes).await
}

/// Patches the status subresource.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch fails.
pub async fn patch_status<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    patch: &Patch<Value>,
) -> Result<(), ReconcileError> {
    api(client, resource)
        .patch_status(
            &kube::ResourceExt::name_any(resource),
            &PatchParams::default(),
            patch,
        )
        .await?;
    Ok(())
}
