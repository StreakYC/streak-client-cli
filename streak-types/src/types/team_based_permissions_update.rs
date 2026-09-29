pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Partial permission update for one system team list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamBasedPermissionsUpdate {
    /// New default permission set. Omit to leave the default unchanged.
    #[serde(rename = "defaultPermissionSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission_set_name: Option<String>,
    /// Complete user override list. Omit to leave overrides unchanged; send an empty list to clear them.
    #[serde(rename = "aclEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_entries: Option<Vec<TeamAclEntryUpdate>>,
}

impl TeamBasedPermissionsUpdate {
    pub fn builder() -> TeamBasedPermissionsUpdateBuilder {
        <TeamBasedPermissionsUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamBasedPermissionsUpdateBuilder {
    default_permission_set_name: Option<String>,
    acl_entries: Option<Vec<TeamAclEntryUpdate>>,
}

impl TeamBasedPermissionsUpdateBuilder {
    pub fn default_permission_set_name(mut self, value: impl Into<String>) -> Self {
        self.default_permission_set_name = Some(value.into());
        self
    }

    pub fn acl_entries(mut self, value: Vec<TeamAclEntryUpdate>) -> Self {
        self.acl_entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamBasedPermissionsUpdate`].
    pub fn build(self) -> Result<TeamBasedPermissionsUpdate, BuildError> {
        Ok(TeamBasedPermissionsUpdate {
            default_permission_set_name: self.default_permission_set_name,
            acl_entries: self.acl_entries,
        })
    }
}
