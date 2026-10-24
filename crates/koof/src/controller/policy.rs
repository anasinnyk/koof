use crate::managed::{DeletionPolicy, ManagedResourceSpec, ManagementAction};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    pub exists: bool,
    pub up_to_date: bool,
    pub deleting: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    None,
    Create,
    Update,
    Delete,
    Finalize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decision {
    pub action: Action,
    pub late_initialize: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum PolicyError {
    #[error("managementPolicies must include Observe (or \"*\")")]
    ObserveNotAllowed,
}

#[must_use]
pub fn orphans_on_delete(management: &ManagedResourceSpec) -> bool {
    !management.allows(ManagementAction::Delete)
        || management.deletion_policy == DeletionPolicy::Orphan
}

/// Decides what to do; see the module table.
///
/// # Errors
///
/// Returns [`PolicyError::ObserveNotAllowed`] when `Observe` is not allowed.
pub fn decide(state: State, management: &ManagedResourceSpec) -> Result<Decision, PolicyError> {
    if !management.allows(ManagementAction::Observe) {
        return Err(PolicyError::ObserveNotAllowed);
    }

    let allowed = |action| management.allows(action);
    let action = match state {
        State {
            deleting: true,
            exists: true,
            ..
        } if !orphans_on_delete(management) => Action::Delete,
        State { deleting: true, .. } => Action::Finalize,
        State { exists: false, .. } if allowed(ManagementAction::Create) => Action::Create,
        State {
            exists: true,
            up_to_date: false,
            ..
        } if allowed(ManagementAction::Update) => Action::Update,
        _ => Action::None,
    };

    Ok(Decision {
        action,
        late_initialize: state.exists
            && !state.deleting
            && allowed(ManagementAction::LateInitialize),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use ManagementAction::{All, Create, Delete, LateInitialize, Observe, Update};
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    const MISSING: State = State {
        exists: false,
        up_to_date: false,
        deleting: false,
    };
    const DRIFTED: State = State {
        exists: true,
        up_to_date: false,
        deleting: false,
    };
    const IN_SYNC: State = State {
        exists: true,
        up_to_date: true,
        deleting: false,
    };
    const DELETING: State = State {
        exists: true,
        up_to_date: true,
        deleting: true,
    };
    const DELETING_GONE: State = State {
        exists: false,
        up_to_date: false,
        deleting: true,
    };

    fn only(actions: &[ManagementAction]) -> ManagedResourceSpec {
        ManagedResourceSpec {
            management_policies: actions.iter().copied().collect(),
            ..ManagedResourceSpec::default()
        }
    }

    fn everything() -> ManagedResourceSpec {
        ManagedResourceSpec::default()
    }

    fn observe_only() -> ManagedResourceSpec {
        only(&[Observe])
    }

    fn without(action: ManagementAction) -> ManagedResourceSpec {
        let all = [Observe, Create, Update, Delete, LateInitialize];
        only(
            &all.into_iter()
                .filter(|candidate| *candidate != action)
                .collect::<Vec<_>>(),
        )
    }

    fn orphan() -> ManagedResourceSpec {
        ManagedResourceSpec {
            deletion_policy: DeletionPolicy::Orphan,
            ..ManagedResourceSpec::default()
        }
    }

    #[rstest]
    #[case::missing_resource_is_created(MISSING, everything(), Action::Create)]
    #[case::drift_is_corrected(DRIFTED, everything(), Action::Update)]
    #[case::matching_resource_is_left_alone(IN_SYNC, everything(), Action::None)]
    #[case::import_never_creates(MISSING, observe_only(), Action::None)]
    #[case::import_never_corrects_drift(DRIFTED, observe_only(), Action::None)]
    #[case::without_create_a_missing_resource_waits(MISSING, without(Create), Action::None)]
    #[case::without_update_drift_stays(DRIFTED, without(Update), Action::None)]
    #[case::deleting_object_deletes_external(DELETING, everything(), Action::Delete)]
    #[case::orphan_policy_keeps_external(DELETING, orphan(), Action::Finalize)]
    #[case::without_delete_keeps_external(DELETING, without(Delete), Action::Finalize)]
    #[case::import_keeps_external_on_delete(DELETING, observe_only(), Action::Finalize)]
    #[case::already_gone_only_finalizes(DELETING_GONE, everything(), Action::Finalize)]
    fn decides_what_to_do(
        #[case] state: State,
        #[case] spec: ManagedResourceSpec,
        #[case] action: Action,
    ) {
        assert_eq!(
            decide(state, &spec).map(|decision| decision.action),
            Ok(action)
        );
    }

    #[rstest]
    #[case::existing_resource_fills_the_spec(IN_SYNC, everything(), true)]
    #[case::drifted_resource_fills_the_spec(DRIFTED, everything(), true)]
    #[case::nothing_to_copy_from_a_missing_resource(MISSING, everything(), false)]
    #[case::not_while_deleting(DELETING, everything(), false)]
    #[case::not_without_the_policy(IN_SYNC, without(LateInitialize), false)]
    #[case::not_for_an_import(IN_SYNC, observe_only(), false)]
    fn late_initializes_only_live_existing_resources(
        #[case] state: State,
        #[case] spec: ManagedResourceSpec,
        #[case] late_initialize: bool,
    ) {
        assert_eq!(
            decide(state, &spec).map(|decision| decision.late_initialize),
            Ok(late_initialize)
        );
    }

    #[test]
    fn observe_is_mandatory() {
        assert_eq!(
            decide(IN_SYNC, &only(&[Create, Update, Delete])),
            Err(PolicyError::ObserveNotAllowed)
        );
    }

    #[test]
    fn wildcard_allows_every_action() {
        assert_eq!(
            decide(MISSING, &only(&[All])).map(|d| d.action),
            Ok(Action::Create)
        );
    }

    #[rstest]
    #[case::delete_policy_deletes(everything(), false)]
    #[case::orphan_policy_orphans(orphan(), true)]
    #[case::missing_delete_permission_orphans(without(Delete), true)]
    fn orphans_on_delete_when_deletion_is_not_allowed(
        #[case] spec: ManagedResourceSpec,
        #[case] orphans: bool,
    ) {
        assert_eq!(orphans_on_delete(&spec), orphans);
    }
}
