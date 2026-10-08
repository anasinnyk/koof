//! Late initialization: copy values the provider chose for fields the user did not
//! set (provider defaults) into `spec.forProvider`, so the spec shows the real
//! state, drift detection does not flap on them, and a changed provider default
//! cannot silently change the resource. Never overwrites a value the user set.
//!
//! Top-level `forProvider` fields only: a partially set object is left alone.

use crate::{error::ReconcileError, json_path, object, resource::ManagedResource};
use serde_json::{Map, Value};

const FOR_PROVIDER: &str = "spec.forProvider";

/// Observed values for late-initializable fields that are unset in the spec. A
/// value that does not fit the spec's type (schemas of request and response can
/// differ) is skipped.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource cannot be serialized.
pub fn candidates<R: ManagedResource>(
    resource: &R,
    observed: &Value,
) -> Result<Map<String, Value>, ReconcileError> {
    let value = serde_json::to_value(resource)?;
    let desired = json_path::get(&value, FOR_PROVIDER);
    let mut fields = Map::new();

    for field in R::descriptor().late_initialize {
        let is_set = desired
            .and_then(|desired| desired.get(*field))
            .is_some_and(|value| !value.is_null());
        if is_set {
            continue;
        }
        let Some(observed_value) = observed.get(*field).filter(|value| !value.is_null()) else {
            continue;
        };
        if fits::<R>(&value, field, observed_value) {
            fields.insert((*field).to_owned(), observed_value.clone());
        } else {
            tracing::debug!(
                field,
                "observed value does not fit the spec type; not late-initialized"
            );
        }
    }
    Ok(fields)
}

/// Whether `resource` still deserializes with `field` set to `candidate`.
fn fits<R: ManagedResource>(resource: &Value, field: &str, candidate: &Value) -> bool {
    let mut trial = resource.clone();
    let Some(for_provider) =
        json_path::get_mut(&mut trial, FOR_PROVIDER).and_then(Value::as_object_mut)
    else {
        return false;
    };
    for_provider.insert(field.to_owned(), candidate.clone());
    serde_json::from_value::<R>(trial).is_ok()
}

/// Writes `fields` into `spec.forProvider` (merge patch with `resourceVersion`).
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch fails (including a conflict with a
/// concurrent change).
pub async fn apply<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    fields: &Map<String, Value>,
) -> Result<(), ReconcileError> {
    let mut patch = Value::Null;
    json_path::set(&mut patch, FOR_PROVIDER, Value::Object(fields.clone()));
    object::patch_guarded(client, resource, patch).await?;
    Ok(())
}
