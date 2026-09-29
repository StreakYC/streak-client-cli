pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One explicit user permission in a team list ACL.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamAclEntryUpdate {
    /// Key for the user receiving the permission.
    #[serde(rename = "userKey")]
    #[serde(default)]
    pub user_key: String,
    /// Permission set assigned to the user.
    #[serde(rename = "permissionSetName")]
    #[serde(default)]
    pub permission_set_name: String,
}

impl TeamAclEntryUpdate {
    pub fn builder() -> TeamAclEntryUpdateBuilder {
        <TeamAclEntryUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamAclEntryUpdateBuilder {
    user_key: Option<String>,
    permission_set_name: Option<String>,
}

impl TeamAclEntryUpdateBuilder {
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());
        self
    }

    pub fn permission_set_name(mut self, value: impl Into<String>) -> Self {
        self.permission_set_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TeamAclEntryUpdate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_key`](TeamAclEntryUpdateBuilder::user_key)
    /// - [`permission_set_name`](TeamAclEntryUpdateBuilder::permission_set_name)
    pub fn build(self) -> Result<TeamAclEntryUpdate, BuildError> {
        Ok(TeamAclEntryUpdate {
            user_key: self.user_key.ok_or_else(|| BuildError::missing_field("user_key"))?,
            permission_set_name: self.permission_set_name.ok_or_else(|| BuildError::missing_field("permission_set_name"))?,
        })
    }
}
