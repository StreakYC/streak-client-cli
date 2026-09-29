pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Custom field definitions for a team's contacts or organizations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TeamFieldSettings {
    /// Custom fields keyed by their stable field key.
    #[serde(default)]
    pub fields: HashMap<String, TeamField>,
}

impl TeamFieldSettings {
    pub fn builder() -> TeamFieldSettingsBuilder {
        <TeamFieldSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamFieldSettingsBuilder {
    fields: Option<HashMap<String, TeamField>>,
}

impl TeamFieldSettingsBuilder {
    pub fn fields(mut self, value: HashMap<String, TeamField>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamFieldSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](TeamFieldSettingsBuilder::fields)
    pub fn build(self) -> Result<TeamFieldSettings, BuildError> {
        Ok(TeamFieldSettings {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}
