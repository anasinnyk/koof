use crate::{error::ReconcileError, external::Observation, resource::ManagedResource};
use serde_json::Value;

/// Desired fields of `resource` that differ from the observation; the desired
/// state is the update request's source (`spec.forProvider` by default).
///
/// # Errors
///
/// Returns [`ReconcileError`] when the resource cannot be serialized.
pub fn drift<R: ManagedResource>(
    resource: &R,
    observation: &Observation,
) -> Result<Vec<String>, ReconcileError> {
    let Some(observed) = &observation.at_provider else {
        return Ok(Vec::new());
    };
    let from = R::descriptor()
        .update
        .as_ref()
        .map_or("/spec/forProvider", |update| update.request.from);
    let value = serde_json::to_value(resource)?;
    Ok(value
        .pointer(from)
        .map_or_else(Vec::new, |desired| drifted(desired, observed)))
}

#[must_use]
pub fn drifted(desired: &Value, observed: &Value) -> Vec<String> {
    let mut paths = Vec::new();
    collect("", desired, observed, &mut paths);
    paths
}

fn collect(path: &str, desired: &Value, observed: &Value, out: &mut Vec<String>) {
    match (desired, observed) {
        (Value::Object(desired), Value::Object(observed)) => {
            for (key, desired_value) in desired {
                if desired_value.is_null() {
                    continue;
                }
                let Some(observed_value) = observed.get(key) else {
                    continue;
                };
                let child = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                collect(&child, desired_value, observed_value, out);
            }
        }
        (Value::Array(desired), Value::Null) if desired.is_empty() => {}
        (Value::Object(desired), Value::Null) if desired.is_empty() => {}
        (Value::Array(desired), Value::Array(observed)) => {
            let differs = desired.len() != observed.len()
                || desired.iter().zip(observed).any(|(desired, observed)| {
                    let mut elements = Vec::new();
                    collect(path, desired, observed, &mut elements);
                    !elements.is_empty()
                });
            if differs {
                out.push(path.to_owned());
            }
        }
        (desired, observed) if !equal(desired, observed) => out.push(path.to_owned()),
        _ => {}
    }
}

fn equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => match (a.as_f64(), b.as_f64()) {
            (Some(a), Some(b)) => (a - b).abs() < f64::EPSILON,
            _ => a == b,
        },
        _ => left == right,
    }
}
