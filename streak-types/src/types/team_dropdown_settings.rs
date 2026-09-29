pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Options configured for a team dropdown field.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamDropdownSettings {
    /// Dropdown options in display order.
    #[serde(default)]
    pub items: Vec<TeamDropdownItem>,
}

impl TeamDropdownSettings {
    pub fn builder() -> TeamDropdownSettingsBuilder {
        <TeamDropdownSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamDropdownSettingsBuilder {
    items: Option<Vec<TeamDropdownItem>>,
}

impl TeamDropdownSettingsBuilder {
    pub fn items(mut self, value: Vec<TeamDropdownItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamDropdownSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](TeamDropdownSettingsBuilder::items)
    pub fn build(self) -> Result<TeamDropdownSettings, BuildError> {
        Ok(TeamDropdownSettings {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
