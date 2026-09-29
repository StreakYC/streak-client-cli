pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A team member's explicit permission for a system list.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamAclEntry {
    /// Key for the user receiving the permission.
    #[serde(rename = "userKey")]
    #[serde(default)]
    pub user_key: String,
    /// Permission set assigned to the user.
    #[serde(rename = "permissionSetName")]
    #[serde(default)]
    pub permission_set_name: String,
    /// Whether the permission is fixed and cannot be edited by the requester.
    #[serde(rename = "isReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
}

impl TeamAclEntry {
    pub fn builder() -> TeamAclEntryBuilder {
        <TeamAclEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamAclEntryBuilder {
    user_key: Option<String>,
    permission_set_name: Option<String>,
    is_read_only: Option<bool>,
}

impl TeamAclEntryBuilder {
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());
        self
    }

    pub fn permission_set_name(mut self, value: impl Into<String>) -> Self {
        self.permission_set_name = Some(value.into());
        self
    }

    pub fn is_read_only(mut self, value: bool) -> Self {
        self.is_read_only = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamAclEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_key`](TeamAclEntryBuilder::user_key)
    /// - [`permission_set_name`](TeamAclEntryBuilder::permission_set_name)
    pub fn build(self) -> Result<TeamAclEntry, BuildError> {
        Ok(TeamAclEntry {
            user_key: self.user_key.ok_or_else(|| BuildError::missing_field("user_key"))?,
            permission_set_name: self.permission_set_name.ok_or_else(|| BuildError::missing_field("permission_set_name"))?,
            is_read_only: self.is_read_only,
        })
    }
}
