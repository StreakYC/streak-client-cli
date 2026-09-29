pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineSharingEntryInput {
    /// Key for the user to share with. Takes precedence over email when both are provided.
    #[serde(rename = "userKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
    /// Email address used to find or create the user when userKey is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Permission set to assign to the user.
    #[serde(rename = "permissionSetName")]
    #[serde(default)]
    pub permission_set_name: String,
}

impl PipelineSharingEntryInput {
    pub fn builder() -> PipelineSharingEntryInputBuilder {
        <PipelineSharingEntryInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineSharingEntryInputBuilder {
    user_key: Option<String>,
    email: Option<String>,
    permission_set_name: Option<String>,
}

impl PipelineSharingEntryInputBuilder {
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn permission_set_name(mut self, value: impl Into<String>) -> Self {
        self.permission_set_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PipelineSharingEntryInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`permission_set_name`](PipelineSharingEntryInputBuilder::permission_set_name)
    pub fn build(self) -> Result<PipelineSharingEntryInput, BuildError> {
        Ok(PipelineSharingEntryInput {
            user_key: self.user_key,
            email: self.email,
            permission_set_name: self.permission_set_name.ok_or_else(|| BuildError::missing_field("permission_set_name"))?,
        })
    }
}
