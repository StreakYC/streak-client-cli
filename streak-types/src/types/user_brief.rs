pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserBrief {
    /// Key for this user.
    #[serde(rename = "userKey")]
    #[serde(default)]
    pub user_key: String,
    /// User email address, when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Short display name for this user, when available.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Full display name for this user, when available.
    #[serde(rename = "fullName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// Given name for this user, when available.
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// Avatar image URL for this user, when available.
    #[serde(rename = "avatarUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
}

impl UserBrief {
    pub fn builder() -> UserBriefBuilder {
        <UserBriefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBriefBuilder {
    user_key: Option<String>,
    email: Option<String>,
    display_name: Option<String>,
    full_name: Option<String>,
    given_name: Option<String>,
    avatar_url: Option<String>,
}

impl UserBriefBuilder {
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());
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

    pub fn avatar_url(mut self, value: impl Into<String>) -> Self {
        self.avatar_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserBrief`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_key`](UserBriefBuilder::user_key)
    pub fn build(self) -> Result<UserBrief, BuildError> {
        Ok(UserBrief {
            user_key: self.user_key.ok_or_else(|| BuildError::missing_field("user_key"))?,
            email: self.email,
            display_name: self.display_name,
            full_name: self.full_name,
            given_name: self.given_name,
            avatar_url: self.avatar_url,
        })
    }
}
