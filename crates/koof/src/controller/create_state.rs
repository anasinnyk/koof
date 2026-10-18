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
