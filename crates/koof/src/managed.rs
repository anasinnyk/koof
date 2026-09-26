use std::collections::BTreeSet;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// An action the controller may take on the external resource
/// (Crossplane `managementPolicies`).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "PascalCase")]
pub enum ManagementAction {
    /// Every action.
    #[serde(rename = "*")]
    All,
    Observe,
    Create,
    Update,
    Delete,
    LateInitialize,
}

/// What happens to the external resource when the managed resource is deleted.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum DeletionPolicy {
    #[default]
    Delete,
    Orphan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ManagedResourceSpec {
    /// Actions the controller may take. `["*"]` (default) allows all; `["Observe"]`
    /// only reads (import without management). `Observe` is always required.
    #[serde(rename = "managementPolicies", default = "default_management_policies")]
    pub management_policies: BTreeSet<ManagementAction>,

    /// `Orphan` keeps the external resource when the managed resource is deleted.
    /// Without `Delete` in `managementPolicies` the resource is always orphaned.
    #[serde(rename = "deletionPolicy", default)]
    pub deletion_policy: DeletionPolicy,
}

impl ManagedResourceSpec {
    /// Whether `action` is allowed by `managementPolicies`.
    #[must_use]
    pub fn allows(&self, action: ManagementAction) -> bool {
        self.management_policies.contains(&ManagementAction::All)
            || self.management_policies.contains(&action)
    }
}

impl Default for ManagedResourceSpec {
    fn default() -> Self {
        Self {
            management_policies: default_management_policies(),
            deletion_policy: DeletionPolicy::Delete,
        }
    }
}

fn default_management_policies() -> BTreeSet<ManagementAction> {
    [ManagementAction::All].into_iter().collect()
}
