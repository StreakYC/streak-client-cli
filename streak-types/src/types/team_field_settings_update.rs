pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Complete custom-field replacement for team contacts or organizations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TeamFieldSettingsUpdate {
    /// Complete custom-field map keyed by stable field key.
    #[serde(default)]
    pub fields: HashMap<String, TeamFieldUpdate>,
}

impl TeamFieldSettingsUpdate {
    pub fn builder() -> TeamFieldSettingsUpdateBuilder {
        <TeamFieldSettingsUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamFieldSettingsUpdateBuilder {
    fields: Option<HashMap<String, TeamFieldUpdate>>,
}

impl TeamFieldSettingsUpdateBuilder {
    pub fn fields(mut self, value: HashMap<String, TeamFieldUpdate>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamFieldSettingsUpdate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](TeamFieldSettingsUpdateBuilder::fields)
    pub fn build(self) -> Result<TeamFieldSettingsUpdate, BuildError> {
        Ok(TeamFieldSettingsUpdate {
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}
