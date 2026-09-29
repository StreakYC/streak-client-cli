pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One option in a team dropdown field.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamDropdownItem {
    /// Stable key for the option.
    #[serde(default)]
    pub key: String,
    /// Display name for the option.
    #[serde(default)]
    pub name: String,
}

impl TeamDropdownItem {
    pub fn builder() -> TeamDropdownItemBuilder {
        <TeamDropdownItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamDropdownItemBuilder {
    key: Option<String>,
    name: Option<String>,
}

impl TeamDropdownItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TeamDropdownItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](TeamDropdownItemBuilder::key)
    /// - [`name`](TeamDropdownItemBuilder::name)
    pub fn build(self) -> Result<TeamDropdownItem, BuildError> {
        Ok(TeamDropdownItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
