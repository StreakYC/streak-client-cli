pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Complete tag list for a team tag field.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamTagSettingsUpdate {
    /// Tags in display order.
    #[serde(default)]
    pub tags: Vec<TeamTagItem>,
}

impl TeamTagSettingsUpdate {
    pub fn builder() -> TeamTagSettingsUpdateBuilder {
        <TeamTagSettingsUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamTagSettingsUpdateBuilder {
    tags: Option<Vec<TeamTagItem>>,
}

impl TeamTagSettingsUpdateBuilder {
    pub fn tags(mut self, value: Vec<TeamTagItem>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamTagSettingsUpdate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tags`](TeamTagSettingsUpdateBuilder::tags)
    pub fn build(self) -> Result<TeamTagSettingsUpdate, BuildError> {
        Ok(TeamTagSettingsUpdate {
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
        })
    }
}
