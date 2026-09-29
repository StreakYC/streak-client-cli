pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One member in a complete team-roster replacement.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamMemberUpdate {
    /// Existing user key. New invitations may supply email instead.
    #[serde(rename = "userKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
    /// Email address used to invite a user when no user key is supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Role for the team member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Invitation state for the team member.
    #[serde(rename = "inviteStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_status: Option<String>,
    /// Email visibility shared with the team. Allowed values are Existence and None.
    #[serde(rename = "emailSharing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_sharing: Option<String>,
}

impl TeamMemberUpdate {
    pub fn builder() -> TeamMemberUpdateBuilder {
        <TeamMemberUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamMemberUpdateBuilder {
    user_key: Option<String>,
    email: Option<String>,
    role: Option<String>,
    invite_status: Option<String>,
    email_sharing: Option<String>,
}

impl TeamMemberUpdateBuilder {
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn invite_status(mut self, value: impl Into<String>) -> Self {
        self.invite_status = Some(value.into());
        self
    }

    pub fn email_sharing(mut self, value: impl Into<String>) -> Self {
        self.email_sharing = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TeamMemberUpdate`].
    pub fn build(self) -> Result<TeamMemberUpdate, BuildError> {
        Ok(TeamMemberUpdate {
            user_key: self.user_key,
            email: self.email,
            role: self.role,
            invite_status: self.invite_status,
            email_sharing: self.email_sharing,
        })
    }
}
