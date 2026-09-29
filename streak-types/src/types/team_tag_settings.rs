pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Tags configured for a team tag field.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamTagSettings {
    /// Tags in display order.
    #[serde(default)]
    pub tags: Vec<TeamTagItem>,
}

impl TeamTagSettings {
    pub fn builder() -> TeamTagSettingsBuilder {
        <TeamTagSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamTagSettingsBuilder {
    tags: Option<Vec<TeamTagItem>>,
}

impl TeamTagSettingsBuilder {
    pub fn tags(mut self, value: Vec<TeamTagItem>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamTagSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tags`](TeamTagSettingsBuilder::tags)
    pub fn build(self) -> Result<TeamTagSettings, BuildError> {
        Ok(TeamTagSettings {
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
        })
    }
}
