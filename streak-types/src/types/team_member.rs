pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A user and their role and invitation state within a team.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TeamMember {
    /// Key for the member's user account.
    #[serde(rename = "userKey")]
    #[serde(default)]
    pub user_key: String,
    /// Role held by the member within the team.
    pub role: Role,
    /// Timestamp when the member was invited, in epoch milliseconds.
    #[serde(rename = "inviteDate")]
    #[serde(default)]
    pub invite_date: i64,
    /// Current state of the team invitation.
    #[serde(rename = "inviteStatus")]
    pub invite_status: InviteStatus,
    /// Display name for the member.
    #[serde(rename = "displayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Email address for the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Full name for the member.
    #[serde(rename = "fullName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// Given name for the member.
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// Profile-image URL for the member.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// Type of the member's Streak user account.
    #[serde(rename = "userType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<String>,
    /// Level of email-sharing access granted to this team member.
    #[serde(rename = "emailSharing")]
    pub email_sharing: EmailSharing,
}

impl TeamMember {
    pub fn builder() -> TeamMemberBuilder {
        <TeamMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamMemberBuilder {
    user_key: Option<String>,
    role: Option<Role>,
    invite_date: Option<i64>,
    invite_status: Option<InviteStatus>,
    display_name: Option<String>,
    email: Option<String>,
    full_name: Option<String>,
    given_name: Option<String>,
    image: Option<String>,
    user_type: Option<String>,
    email_sharing: Option<EmailSharing>,
}

impl TeamMemberBuilder {
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());
        self
    }

    pub fn role(mut self, value: Role) -> Self {
        self.role = Some(value);
        self
    }

    pub fn invite_date(mut self, value: i64) -> Self {
        self.invite_date = Some(value);
        self
    }

    pub fn invite_status(mut self, value: InviteStatus) -> Self {
        self.invite_status = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
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

    pub fn email_sharing(mut self, value: EmailSharing) -> Self {
        self.email_sharing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_key`](TeamMemberBuilder::user_key)
    /// - [`role`](TeamMemberBuilder::role)
    /// - [`invite_date`](TeamMemberBuilder::invite_date)
    /// - [`invite_status`](TeamMemberBuilder::invite_status)
    /// - [`email_sharing`](TeamMemberBuilder::email_sharing)
    pub fn build(self) -> Result<TeamMember, BuildError> {
        Ok(TeamMember {
            user_key: self.user_key.ok_or_else(|| BuildError::missing_field("user_key"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            invite_date: self.invite_date.ok_or_else(|| BuildError::missing_field("invite_date"))?,
            invite_status: self.invite_status.ok_or_else(|| BuildError::missing_field("invite_status"))?,
            display_name: self.display_name,
            email: self.email,
            full_name: self.full_name,
            given_name: self.given_name,
            image: self.image,
            user_type: self.user_type,
            email_sharing: self.email_sharing.ok_or_else(|| BuildError::missing_field("email_sharing"))?,
        })
    }
}
