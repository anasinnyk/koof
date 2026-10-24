use crate::{error::ReconcileError, object, resource::ManagedResource};
use serde_json::{Map, Value};

const FOR_PROVIDER: &str = "/spec/forProvider";

/// Observed values for late-initializable fields that are unset in the spec. A
/// value that does not fit the spec's type (schemas of request and response can
/// differ) is skipped, and so is one that the spec type cannot represent as set
/// (e.g. an empty list skipped when serialized): writing it would change nothing
/// and late initialization would repeat forever.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource cannot be serialized.
pub fn candidates<R: ManagedResource>(
    resource: &R,
    observed: &Value,
) -> Result<Map<String, Value>, ReconcileError> {
    let value = serde_json::to_value(resource)?;
    let desired = value.pointer(FOR_PROVIDER);
    let mut fields = Map::new();

    for field in R::descriptor().late_initialize {
        let is_set = desired
            .and_then(|desired| desired.get(*field))
            .is_some_and(|value| !value.is_null());
        if is_set {
            continue;
        }
        let Some(observed_value) = observed
            .get(*field)
            .filter(|value| !value.is_null())
            .map(without_nulls)
        else {
            continue;
        };
        match trial::<R>(&value, field, &observed_value) {
            Trial::Changes => {
                fields.insert((*field).to_owned(), observed_value);
            }
            Trial::NoChange => {}
            Trial::DoesNotFit => tracing::debug!(
                field,
                "observed value does not fit the spec type; not late-initialized"
            ),
        }
    }
    Ok(fields)
}

fn without_nulls(value: &Value) -> Value {
    match value {
        Value::Object(object) => Value::Object(
            object
                .iter()
                .filter(|(_, member)| !member.is_null())
                .map(|(key, member)| (key.clone(), without_nulls(member)))
                .collect(),
        ),
        Value::Array(items) => Value::Array(items.iter().map(without_nulls).collect()),
        other => other.clone(),
    }
}

enum Trial {
    Changes,
    NoChange,
    DoesNotFit,
}

fn trial<R: ManagedResource>(resource: &Value, field: &str, candidate: &Value) -> Trial {
    let mut trial = resource.clone();
    let Some(for_provider) = trial
        .pointer_mut(FOR_PROVIDER)
        .and_then(Value::as_object_mut)
    else {
        return Trial::DoesNotFit;
    };
    for_provider.insert(field.to_owned(), candidate.clone());
    match serde_json::from_value::<R>(trial).map(|typed| serde_json::to_value(typed)) {
        Ok(Ok(round_tripped)) if round_tripped == *resource => Trial::NoChange,
        Ok(Ok(_)) => Trial::Changes,
        Ok(Err(_)) | Err(_) => Trial::DoesNotFit,
    }
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
    let patch = serde_json::json!({ "spec": { "forProvider": fields } });
    object::patch_guarded(client, resource, patch).await?;
    Ok(())
}
