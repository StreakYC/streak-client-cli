pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamCreateBody {
    /// Display name for the new team.
    #[serde(default)]
    pub name: String,
    /// Initial team members. The authenticated creator must be included.
    #[serde(default)]
    pub members: Vec<TeamMemberCreate>,
}

impl TeamCreateBody {
    pub fn builder() -> TeamCreateBodyBuilder {
        <TeamCreateBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamCreateBodyBuilder {
    name: Option<String>,
    members: Option<Vec<TeamMemberCreate>>,
}

impl TeamCreateBodyBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn members(mut self, value: Vec<TeamMemberCreate>) -> Self {
        self.members = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamCreateBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](TeamCreateBodyBuilder::name)
    /// - [`members`](TeamCreateBodyBuilder::members)
    pub fn build(self) -> Result<TeamCreateBody, BuildError> {
        Ok(TeamCreateBody {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            members: self.members.ok_or_else(|| BuildError::missing_field("members"))?,
        })
    }
}

