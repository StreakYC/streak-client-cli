pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Complete option list for a team dropdown field.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamDropdownSettingsUpdate {
    /// Dropdown options in display order.
    #[serde(default)]
    pub items: Vec<TeamDropdownItem>,
}

impl TeamDropdownSettingsUpdate {
    pub fn builder() -> TeamDropdownSettingsUpdateBuilder {
        <TeamDropdownSettingsUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamDropdownSettingsUpdateBuilder {
    items: Option<Vec<TeamDropdownItem>>,
}

impl TeamDropdownSettingsUpdateBuilder {
    pub fn items(mut self, value: Vec<TeamDropdownItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamDropdownSettingsUpdate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](TeamDropdownSettingsUpdateBuilder::items)
    pub fn build(self) -> Result<TeamDropdownSettingsUpdate, BuildError> {
        Ok(TeamDropdownSettingsUpdate {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
