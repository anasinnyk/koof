//! Drift detection: does the external resource match `spec.forProvider`?
//!
//! Only fields set in the desired state are compared, and only where the observed
//! state has them: a field the API does not return (write-only, not selected into
//! `atProvider`, references such as `zoneRef`) cannot be checked and is skipped.
//! Unset desired fields are late initialization's business, not drift.

use serde_json::Value;

/// Dot paths of desired fields that differ from the observed state; empty when up
/// to date.
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
        (desired, observed) if !equal(desired, observed) => out.push(path.to_owned()),
        _ => {}
    }
}

/// JSON equality where numbers compare by value (`300` equals `300.0`).
fn equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => match (a.as_f64(), b.as_f64()) {
            (Some(a), Some(b)) => (a - b).abs() < f64::EPSILON,
            _ => a == b,
        },
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b).all(|(a, b)| equal(a, b))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len()
                && a.iter()
                    .all(|(key, value)| b.get(key).is_some_and(|other| equal(value, other)))
        }
        _ => left == right,
    }
}
