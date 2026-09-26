//! The reconcile decision as a pure function of observed state and policies.
//!
//! | deleting | exists | up to date | → action                                   |
//! |----------|--------|------------|--------------------------------------------|
//! | yes      | yes    | –          | `Delete` if allowed and `deletionPolicy: Delete`, else `Finalize` (orphan) |
//! | yes      | no     | –          | `Finalize`                                 |
//! | no       | no     | –          | `Create` if allowed, else `None`           |
//! | no       | yes    | no         | `Update` if allowed, else `None`           |
//! | no       | yes    | yes        | `None`                                     |
//!
//! Late initialization runs alongside when the resource exists, is not being
//! deleted and `LateInitialize` is allowed. `Observe` must always be allowed.

use crate::managed::{DeletionPolicy, ManagedResourceSpec, ManagementAction};

/// What the controller knows after observing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct State {
    /// The external resource exists.
    pub exists: bool,
    /// `spec.forProvider` matches the external resource (meaningful when it exists).
    pub up_to_date: bool,
    /// The managed resource has a deletion timestamp.
    pub deleting: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Nothing to change externally.
    None,
    Create,
    Update,
    Delete,
    /// Deleting, and nothing (more) to do externally — the resource is gone or is
    /// orphaned: release the managed resource (remove the finalizer).
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
        } if allowed(ManagementAction::Delete)
            && management.deletion_policy == DeletionPolicy::Delete =>
        {
            Action::Delete
        }
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
