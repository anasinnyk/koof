use k8s_openapi::apimachinery::pkg::apis::meta::v1::{Condition, Time};

pub const READY: &str = "Ready";
pub const SYNCED: &str = "Synced";

const MAX_MESSAGE: usize = 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionSpec {
    type_: &'static str,
    status: &'static str,
    reason: &'static str,
    message: String,
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
    pub fn in_use(message: impl Into<String>) -> Self {
        Self::new(READY, "False", "InUse", message)
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
        Self {
            type_,
            status,
            reason,
            message: truncate(message.into()),
        }
    }
}

#[must_use]
pub fn truncate(mut message: String) -> String {
    if message.len() > MAX_MESSAGE {
        let mut end = MAX_MESSAGE - '…'.len_utf8();
        while !message.is_char_boundary(end) {
            end -= 1;
        }
        message.truncate(end);
        message.push('…');
    }
    message
}

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

#[must_use]
pub fn is_true(conditions: &[Condition], type_: &str) -> bool {
    conditions
        .iter()
        .any(|condition| condition.type_ == type_ && condition.status == "True")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    fn condition(type_: &str, status: &str, since: &str) -> Condition {
        Condition {
            type_: type_.to_owned(),
            status: status.to_owned(),
            reason: "Before".to_owned(),
            message: String::new(),
            observed_generation: Some(1),
            last_transition_time: Time::from(since.parse::<jiff::Timestamp>().expect("timestamp")),
        }
    }

    #[rstest]
    #[case::short_message_is_kept("not found", "not found")]
    #[case::message_at_the_limit_is_kept(&"x".repeat(1024), &"x".repeat(1024))]
    #[case::long_message_is_cut_with_an_ellipsis(&"x".repeat(2000), &format!("{}…", "x".repeat(1021)))]
    #[case::cut_never_splits_a_character(&"ї".repeat(600), &format!("{}…", "ї".repeat(510)))]
    fn messages_fit_into_a_condition(#[case] message: &str, #[case] expected: &str) {
        assert_eq!(truncate(message.to_owned()), expected);
    }

    #[test]
    fn set_replaces_the_condition_of_the_same_type() {
        let before = [condition(READY, "False", "2026-01-01T10:00:00Z")];

        let after = set(&before, ConditionSpec::available(), Some(2));

        assert_eq!(
            (
                after.len(),
                after[0].status.as_str(),
                after[0].reason.as_str(),
                after[0].observed_generation
            ),
            (1, "True", "Available", Some(2))
        );
    }

    #[test]
    fn set_keeps_the_transition_time_while_the_status_holds() {
        let before = [condition(READY, "True", "2026-01-01T10:00:00Z")];

        let after = set(&before, ConditionSpec::available(), Some(2));

        assert_eq!(
            after[0].last_transition_time,
            before[0].last_transition_time
        );
    }

    #[test]
    fn set_moves_the_transition_time_when_the_status_changes() {
        let before = [condition(READY, "True", "2026-01-01T10:00:00Z")];

        let after = set(&before, ConditionSpec::deleting(), Some(2));

        assert_ne!(
            after[0].last_transition_time,
            before[0].last_transition_time
        );
    }

    #[test]
    fn set_keeps_other_conditions_sorted_by_type() {
        let before = [condition(SYNCED, "True", "2026-01-01T10:00:00Z")];

        let after = set(&before, ConditionSpec::creating(), None);

        let types: Vec<&str> = after
            .iter()
            .map(|condition| condition.type_.as_str())
            .collect();
        assert_eq!(types, [READY, SYNCED]);
    }

    #[test]
    fn set_truncates_long_messages() {
        let after = set(&[], ConditionSpec::reconcile_error("x".repeat(5000)), None);

        assert_eq!(after[0].message.len(), 1024);
    }

    #[rstest]
    #[case::true_condition(&[condition(READY, "True", "2026-01-01T10:00:00Z")], true)]
    #[case::false_condition(&[condition(READY, "False", "2026-01-01T10:00:00Z")], false)]
    #[case::other_condition_true(&[condition(SYNCED, "True", "2026-01-01T10:00:00Z")], false)]
    #[case::no_conditions(&[], false)]
    fn is_true_checks_one_condition_type(#[case] conditions: &[Condition], #[case] ready: bool) {
        assert_eq!(is_true(conditions, READY), ready);
    }
}
