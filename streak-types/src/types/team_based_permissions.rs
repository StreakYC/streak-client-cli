pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Default and user-specific permissions for a system team list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamBasedPermissions {
    /// Permission set applied to team members without an explicit override.
    #[serde(rename = "defaultPermissionSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission_set_name: Option<String>,
    /// User permission overrides in the wire shape used by team sharing controls.
    #[serde(rename = "aclEntries")]
    #[serde(default)]
    pub acl_entries: Vec<TeamAclEntry>,
}

impl TeamBasedPermissions {
    pub fn builder() -> TeamBasedPermissionsBuilder {
        <TeamBasedPermissionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamBasedPermissionsBuilder {
    default_permission_set_name: Option<String>,
    acl_entries: Option<Vec<TeamAclEntry>>,
}

impl TeamBasedPermissionsBuilder {
    pub fn default_permission_set_name(mut self, value: impl Into<String>) -> Self {
        self.default_permission_set_name = Some(value.into());
        self
    }

    pub fn acl_entries(mut self, value: Vec<TeamAclEntry>) -> Self {
        self.acl_entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamBasedPermissions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`acl_entries`](TeamBasedPermissionsBuilder::acl_entries)
    pub fn build(self) -> Result<TeamBasedPermissions, BuildError> {
        Ok(TeamBasedPermissions {
            default_permission_set_name: self.default_permission_set_name,
            acl_entries: self.acl_entries.ok_or_else(|| BuildError::missing_field("acl_entries"))?,
        })
    }
}
