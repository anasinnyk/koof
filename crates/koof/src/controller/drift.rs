use crate::external::{Observation, resolve::Node};
use serde_json::Value;

/// Desired fields of `node` (a resource with its item references resolved) that
/// differ from the observation; the desired state is the update request's source
/// (`spec.forProvider` by default).
#[must_use]
pub fn drift(node: &Node, observation: &Observation) -> Vec<String> {
    let Some(observed) = &observation.at_provider else {
        return Vec::new();
    };
    let descriptor = node.descriptor;
    let from = descriptor
        .update
        .as_ref()
        .map_or("/spec/forProvider", |update| update.request.from);
    node.value.pointer(from).map_or_else(Vec::new, |desired| {
        drifted(desired, observed, descriptor.list_map_keys)
    })
}

/// Paths of the `desired` fields that differ from `observed`. Lists in
/// `list_map_keys` are matched item by item on those keys, in any order; other lists
/// are compared by position.
#[must_use]
pub fn drifted(desired: &Value, observed: &Value, list_map_keys: &[ListMapKeys]) -> Vec<String> {
    let mut paths = Vec::new();
    Comparison { list_map_keys }.collect("", desired, observed, &mut paths);
    paths
}

type ListMapKeys = (&'static str, &'static [&'static str]);

struct Comparison<'a> {
    list_map_keys: &'a [ListMapKeys],
}

impl Comparison<'_> {
    fn collect(&self, path: &str, desired: &Value, observed: &Value, out: &mut Vec<String>) {
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
                    self.collect(&child, desired_value, observed_value, out);
                }
            }
            (Value::Array(desired), Value::Null) if desired.is_empty() => {}
            (Value::Object(desired), Value::Null) if desired.is_empty() => {}
            (Value::Array(desired), Value::Array(observed)) => {
                if desired.len() != observed.len() || self.items_differ(path, desired, observed) {
                    out.push(path.to_owned());
                }
            }
            (desired, observed) if !equal(desired, observed) => out.push(path.to_owned()),
            _ => {}
        }
    }

    fn items_differ(&self, path: &str, desired: &[Value], observed: &[Value]) -> bool {
        let differs = |desired: &Value, observed: &Value| {
            let mut elements = Vec::new();
            self.collect(path, desired, observed, &mut elements);
            !elements.is_empty()
        };
        let keys = self
            .list_map_keys
            .iter()
            .find(|(field, _)| *field == path)
            .map(|(_, keys)| *keys);
        match keys {
            Some(keys) => desired.iter().any(|desired| {
                observed
                    .iter()
                    .find(|observed| keys.iter().all(|key| desired.get(key) == observed.get(key)))
                    .is_none_or(|observed| differs(desired, observed))
            }),
            None => desired
                .iter()
                .zip(observed)
                .any(|(desired, observed)| differs(desired, observed)),
        }
    }
}

fn equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) if a.is_f64() || b.is_f64() => {
            match (a.as_f64(), b.as_f64()) {
                (Some(a), Some(b)) => (a - b).abs() <= f64::EPSILON * a.abs().max(b.abs()),
                _ => false,
            }
        }
        _ => left == right,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;
    use serde_json::json;

    #[rstest]
    #[case::equal_values(json!({ "ttl": 300 }), json!({ "ttl": 300 }))]
    #[case::fields_the_user_did_not_set(json!({ "ttl": null }), json!({ "ttl": 300 }))]
    #[case::fields_the_api_does_not_return(json!({ "comment": "x" }), json!({}))]
    #[case::read_only_fields_the_api_adds(json!({ "ttl": 300 }), json!({ "ttl": 300, "id": "1" }))]
    #[case::empty_list_and_null(json!({ "tags": [] }), json!({ "tags": null }))]
    #[case::empty_object_and_null(json!({ "settings": {} }), json!({ "settings": null }))]
    #[case::float_rounding(json!({ "weight": 0.3 }), json!({ "weight": 0.1 + 0.2 }))]
    #[case::integer_written_as_float(json!({ "ttl": 300 }), json!({ "ttl": 300.0 }))]
    #[case::extra_fields_inside_list_items(
        json!({ "rules": [{ "action": "allow" }] }),
        json!({ "rules": [{ "action": "allow", "id": "r1" }] })
    )]
    #[case::keyed_list_in_another_order(
        json!({ "teamRoles": [{ "teamSlug": "a", "role": "admin" }, { "teamSlug": "b" }] }),
        json!({ "teamRoles": [{ "teamSlug": "b", "role": null }, { "teamSlug": "a", "role": "admin" }] })
    )]
    fn is_not_drift(#[case] desired: Value, #[case] observed: Value) {
        assert_eq!(drifted(&desired, &observed, KEYED), Vec::<String>::new());
    }

    const KEYED: &[ListMapKeys] = &[("teamRoles", &["teamSlug"])];

    #[rstest]
    #[case::changed_value(json!({ "ttl": 300 }), json!({ "ttl": 60 }), &["ttl"])]
    #[case::changed_nested_value(
        json!({ "settings": { "ipv4_only": true } }),
        json!({ "settings": { "ipv4_only": false } }),
        &["settings.ipv4_only"]
    )]
    #[case::large_integers_compared_exactly(
        json!({ "id": 9_007_199_254_740_993_u64 }),
        json!({ "id": 9_007_199_254_740_992_u64 }),
        &["id"]
    )]
    #[case::list_with_another_length(json!({ "tags": ["a"] }), json!({ "tags": ["a", "b"] }), &["tags"])]
    #[case::list_in_another_order(json!({ "tags": ["a", "b"] }), json!({ "tags": ["b", "a"] }), &["tags"])]
    #[case::changed_field_in_a_list_item(
        json!({ "rules": [{ "action": "allow" }] }),
        json!({ "rules": [{ "action": "block" }] }),
        &["rules"]
    )]
    #[case::every_changed_field(
        json!({ "ttl": 300, "proxied": true }),
        json!({ "ttl": 60, "proxied": false }),
        &["proxied", "ttl"]
    )]
    #[case::keyed_list_with_another_item(
        json!({ "teamRoles": [{ "teamSlug": "a" }, { "teamSlug": "b" }] }),
        json!({ "teamRoles": [{ "teamSlug": "a" }, { "teamSlug": "c" }] }),
        &["teamRoles"]
    )]
    #[case::keyed_list_with_a_changed_item(
        json!({ "teamRoles": [{ "teamSlug": "a", "role": "admin" }] }),
        json!({ "teamRoles": [{ "teamSlug": "a", "role": "contributor" }] }),
        &["teamRoles"]
    )]
    #[case::keyed_list_with_a_missing_item(
        json!({ "teamRoles": [{ "teamSlug": "a" }] }),
        json!({ "teamRoles": [{ "teamSlug": "a" }, { "teamSlug": "b" }] }),
        &["teamRoles"]
    )]
    fn is_drift(#[case] desired: Value, #[case] observed: Value, #[case] fields: &[&str]) {
        assert_eq!(drifted(&desired, &observed, KEYED), fields);
    }
}
