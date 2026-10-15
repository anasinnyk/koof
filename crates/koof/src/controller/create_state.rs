use crate::{error::ReconcileError, object, resource::ManagedResource};
use serde_json::{Map, Value};
use std::time::Duration;

pub const CREATE_PENDING: &str = "external-create-pending";
pub const CREATE_SUCCEEDED: &str = "external-create-succeeded";
pub const CREATE_FAILED: &str = "external-create-failed";

pub const CREATE_GRACE: Duration = Duration::from_mins(1);

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
    object::patch_guarded(
        client,
        resource,
        serde_json::json!({ "metadata": { "annotations": annotations } }),
    )
    .await
}

#[must_use]
pub fn now() -> String {
    jiff::Timestamp::now().to_string()
}
