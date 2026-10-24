use crate::{
    error::ReconcileError,
    object,
    resource::ManagedResource,
    retry::{self, RetryPolicy},
};
use serde_json::{Map, Value};
use std::time::Duration;

pub const CREATE_PENDING: &str = "external-create-pending";
pub const CREATE_SUCCEEDED: &str = "external-create-succeeded";
pub const CREATE_FAILED: &str = "external-create-failed";

pub const CREATE_GRACE: Duration = Duration::from_mins(1);

const RECORD_RETRY: RetryPolicy = RetryPolicy {
    base: Duration::from_millis(200),
    max: Duration::from_secs(2),
    attempts: Some(5),
    waiting: None,
    permanent: None,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CreateState {
    pub pending: Option<jiff::Timestamp>,
    pub succeeded: Option<jiff::Timestamp>,
    pub failed: Option<jiff::Timestamp>,
}

impl CreateState {
    #[must_use]
    pub fn of<R: ManagedResource>(resource: &R) -> Self {
        let descriptor = R::descriptor();
        let annotations = kube::ResourceExt::annotations(resource);
        let timestamp = |name: &str| {
            annotations
                .get(&descriptor.annotation(name))
                .and_then(|value| value.parse::<jiff::Timestamp>().ok())
        };
        Self {
            pending: timestamp(CREATE_PENDING),
            succeeded: timestamp(CREATE_SUCCEEDED),
            failed: timestamp(CREATE_FAILED),
        }
    }

    #[must_use]
    pub fn is_incomplete(&self) -> bool {
        self.pending.is_some_and(|pending| {
            self.succeeded.is_none_or(|succeeded| succeeded < pending)
                && self.failed.is_none_or(|failed| failed < pending)
        })
    }

    #[must_use]
    pub fn recently_succeeded(&self, now: jiff::Timestamp) -> bool {
        self.succeeded.is_some_and(|succeeded| {
            now.duration_since(succeeded)
                .unsigned_abs()
                .lt(&CREATE_GRACE)
        })
    }
}

/// Sets (or with `None`, removes) koof annotations; returns the updated object.
/// The patch carries `resourceVersion`, so a concurrent change fails with a
/// conflict instead of racing.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch fails.
pub async fn annotate<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    values: &[(&str, Option<String>)],
) -> Result<R, ReconcileError> {
    object::patch_guarded(client, resource, annotations::<R>(values)).await
}

/// [`annotate`] without `resourceVersion`, retried on transient errors, so a
/// concurrent change cannot lose the outcome of a create.
///
/// # Errors
///
/// Returns [`ReconcileError`] when the patch still fails after the retries.
pub async fn record<R: ManagedResource>(
    client: &kube::Client,
    resource: &R,
    values: &[(&str, Option<String>)],
) -> Result<(), ReconcileError> {
    let patch = annotations::<R>(values);
    retry::with_retries(&RECORD_RETRY, || object::patch(client, resource, &patch)).await?;
    Ok(())
}

fn annotations<R: ManagedResource>(values: &[(&str, Option<String>)]) -> Value {
    let descriptor = R::descriptor();
    let annotations: Map<String, Value> = values
        .iter()
        .map(|(name, value)| {
            (
                descriptor.annotation(name),
                value.clone().map_or(Value::Null, Value::String),
            )
        })
        .collect();
    serde_json::json!({ "metadata": { "annotations": annotations } })
}

#[must_use]
pub fn now() -> String {
    jiff::Timestamp::now().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn at(time: &str) -> jiff::Timestamp {
        format!("2026-01-01T{time}Z").parse().expect("timestamp")
    }

    fn state(pending: Option<&str>, succeeded: Option<&str>, failed: Option<&str>) -> CreateState {
        CreateState {
            pending: pending.map(at),
            succeeded: succeeded.map(at),
            failed: failed.map(at),
        }
    }

    #[rstest]
    #[case::never_created(state(None, None, None), false)]
    #[case::create_sent_and_no_outcome(state(Some("10:00:00"), None, None), true)]
    #[case::create_succeeded(state(Some("10:00:00"), Some("10:00:01"), None), false)]
    #[case::create_failed(state(Some("10:00:00"), None, Some("10:00:01")), false)]
    #[case::new_create_after_a_success(state(Some("11:00:00"), Some("10:00:01"), None), true)]
    #[case::new_create_after_a_failure(state(Some("11:00:00"), None, Some("10:00:01")), true)]
    #[case::retried_after_a_failure_and_succeeded(
        state(Some("11:00:00"), Some("11:00:01"), Some("10:00:01")),
        false
    )]
    fn create_with_unknown_outcome_blocks_another_create(
        #[case] state: CreateState,
        #[case] incomplete: bool,
    ) {
        assert_eq!(state.is_incomplete(), incomplete);
    }

    #[rstest]
    #[case::just_created("10:00:30", true)]
    #[case::still_within_the_grace_period("10:00:59", true)]
    #[case::grace_period_over("10:01:00", false)]
    #[case::long_ago("12:00:00", false)]
    fn freshly_created_resource_may_not_be_visible_yet(#[case] now: &str, #[case] recent: bool) {
        let created = state(Some("09:59:59"), Some("10:00:00"), None);
        assert_eq!(created.recently_succeeded(at(now)), recent);
    }

    #[test]
    fn never_created_resource_is_not_recent() {
        assert!(!CreateState::default().recently_succeeded(at("10:00:00")));
    }
}
