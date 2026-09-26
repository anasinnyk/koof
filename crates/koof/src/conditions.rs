//! Crossplane-style conditions: `Ready` (is the external resource there and usable)
//! and `Synced` (did the last reconcile succeed).

use k8s_openapi::apimachinery::pkg::apis::meta::v1::{Condition, Time};

pub const READY: &str = "Ready";
pub const SYNCED: &str = "Synced";

/// Longest message stored in a condition; provider error bodies can be large.
const MAX_MESSAGE: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionSpec {
    pub type_: &'static str,
    pub status: &'static str,
    pub reason: &'static str,
    pub message: String,
}

impl ConditionSpec {
    #[must_use]
    pub fn available() -> Self {
        Self::new(READY, "True", "Available", "External resource is available")
    }

    #[must_use]
    pub fn unavailable(message: impl Into<String>) -> Self {
        Self::new(READY, "False", "Unavailable", message)
    }

    #[must_use]
    pub fn creating() -> Self {
        Self::new(
            READY,
            "False",
            "Creating",
            "External resource is being created",
        )
    }

    #[must_use]
    pub fn deleting() -> Self {
        Self::new(
            READY,
            "False",
            "Deleting",
            "External resource is being deleted",
        )
    }

    #[must_use]
    pub fn reconcile_success() -> Self {
        Self::new(SYNCED, "True", "ReconcileSuccess", "")
    }

    #[must_use]
    pub fn reconcile_error(message: impl Into<String>) -> Self {
        Self::new(SYNCED, "False", "ReconcileError", message)
    }

    fn new(
        type_: &'static str,
        status: &'static str,
        reason: &'static str,
        message: impl Into<String>,
    ) -> Self {
        let mut message = message.into();
        if message.len() > MAX_MESSAGE {
            let mut end = MAX_MESSAGE;
            while !message.is_char_boundary(end) {
                end -= 1;
            }
            message.truncate(end);
            message.push('…');
        }
        Self {
            type_,
            status,
            reason,
            message,
        }
    }
}

/// `conditions` with `spec` inserted or replaced. `lastTransitionTime` is kept when
/// the status does not change.
#[must_use]
pub fn set(
    conditions: &[Condition],
    spec: ConditionSpec,
    generation: Option<i64>,
) -> Vec<Condition> {
    let previous = conditions
        .iter()
        .find(|condition| condition.type_ == spec.type_);
    let last_transition_time = previous
        .filter(|condition| condition.status == spec.status)
        .map_or_else(
            || Time::from(jiff::Timestamp::now()),
            |condition| condition.last_transition_time.clone(),
        );

    let updated = Condition {
        type_: spec.type_.to_owned(),
        status: spec.status.to_owned(),
        reason: spec.reason.to_owned(),
        message: spec.message,
        observed_generation: generation,
        last_transition_time,
    };

    let mut result: Vec<Condition> = conditions
        .iter()
        .filter(|condition| condition.type_ != spec.type_)
        .cloned()
        .collect();
    result.push(updated);
    result.sort_by(|a, b| a.type_.cmp(&b.type_));
    result
}

/// Whether `conditions` has `type_` with status `True`.
#[must_use]
pub fn is_true(conditions: &[Condition], type_: &str) -> bool {
    conditions
        .iter()
        .any(|condition| condition.type_ == type_ && condition.status == "True")
}
