pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A user to add when creating a team.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamMemberCreate {
    /// Existing user key. Supply either this property or email.
    #[serde(rename = "userKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
    /// Email address used to find or create the user when userKey is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl TeamMemberCreate {
    pub fn builder() -> TeamMemberCreateBuilder {
        <TeamMemberCreateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamMemberCreateBuilder {
    user_key: Option<String>,
    email: Option<String>,
}

impl TeamMemberCreateBuilder {
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TeamMemberCreate`].
    pub fn build(self) -> Result<TeamMemberCreate, BuildError> {
        Ok(TeamMemberCreate {
            user_key: self.user_key,
            email: self.email,
        })
    }
}
