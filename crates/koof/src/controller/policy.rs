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
