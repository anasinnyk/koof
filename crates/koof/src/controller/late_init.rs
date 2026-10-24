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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::record;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use serde_json::json;

    fn late_initialized(spec: Value, observed: &Value) -> Value {
        Value::Object(candidates(&record(None, spec), observed).expect("candidates"))
    }

    #[rstest]
    #[case::copies_a_value_the_user_left_unset(json!({}), json!({ "ttl": 300 }), json!({ "ttl": 300 }))]
    #[case::copies_every_unset_field(
        json!({ "proxied": true }),
        json!({ "name": "www.example.com", "ttl": 300 }),
        json!({ "name": "www.example.com", "ttl": 300 })
    )]
    #[case::keeps_what_the_user_set(json!({ "ttl": 60 }), json!({ "ttl": 300 }), json!({}))]
    #[case::ignores_fields_not_marked_for_late_initialization(json!({}), json!({ "proxied": true }), json!({}))]
    #[case::ignores_values_the_api_does_not_have(json!({}), json!({ "ttl": null }), json!({}))]
    #[case::ignores_values_that_do_not_fit_the_spec(json!({}), json!({ "ttl": "automatic" }), json!({}))]
    #[case::ignores_values_the_spec_cannot_show_as_set(json!({}), json!({ "tags": [] }), json!({}))]
    #[case::drops_nulls_inside_copied_objects(
        json!({}),
        json!({ "settings": { "ipv4_only": true, "flatten_cname": null } }),
        json!({ "settings": { "ipv4_only": true } })
    )]
    fn fills_unset_spec_fields_from_the_api(
        #[case] spec: Value,
        #[case] observed: Value,
        #[case] fields: Value,
    ) {
        assert_eq!(late_initialized(spec, &observed), fields);
    }
}
