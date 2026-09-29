pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineSharingEntry {
    /// Key for the shared user.
    #[serde(rename = "userKey")]
    #[serde(default)]
    pub user_key: String,
    /// Permission set assigned to the user.
    #[serde(rename = "permissionSetName")]
    #[serde(default)]
    pub permission_set_name: String,
    /// User email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User display name.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// User full name.
    #[serde(rename = "fullName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// User given name.
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// User profile image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// User account type.
    #[serde(rename = "userType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    /// Whether the sharing entry is read-only.
    #[serde(rename = "isReadOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read_only: Option<bool>,
}

impl PipelineSharingEntry {
    pub fn builder() -> PipelineSharingEntryBuilder {
        <PipelineSharingEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineSharingEntryBuilder {
    user_key: Option<String>,
    permission_set_name: Option<String>,
    email: Option<String>,
    display_name: Option<String>,
    full_name: Option<String>,
    given_name: Option<String>,
    image: Option<String>,
    user_type: Option<String>,
    is_read_only: Option<bool>,
}

impl PipelineSharingEntryBuilder {
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());
        self
    }

    pub fn permission_set_name(mut self, value: impl Into<String>) -> Self {
        self.permission_set_name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn full_name(mut self, value: impl Into<String>) -> Self {
        self.full_name = Some(value.into());
        self
    }

    pub fn given_name(mut self, value: impl Into<String>) -> Self {
        self.given_name = Some(value.into());
        self
    }

    pub fn image(mut self, value: impl Into<String>) -> Self {
        self.image = Some(value.into());
        self
    }

    pub fn user_type(mut self, value: impl Into<String>) -> Self {
        self.user_type = Some(value.into());
        self
    }

    pub fn is_read_only(mut self, value: bool) -> Self {
        self.is_read_only = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineSharingEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_key`](PipelineSharingEntryBuilder::user_key)
    /// - [`permission_set_name`](PipelineSharingEntryBuilder::permission_set_name)
    pub fn build(self) -> Result<PipelineSharingEntry, BuildError> {
        Ok(PipelineSharingEntry {
            user_key: self.user_key.ok_or_else(|| BuildError::missing_field("user_key"))?,
            permission_set_name: self.permission_set_name.ok_or_else(|| BuildError::missing_field("permission_set_name"))?,
            email: self.email,
            display_name: self.display_name,
            full_name: self.full_name,
            given_name: self.given_name,
            image: self.image,
            user_type: self.user_type,
            is_read_only: self.is_read_only,
        })
    }
}
