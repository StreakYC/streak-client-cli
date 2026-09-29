pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One tag configured for a team tag field.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeamTagItem {
    /// Stable key for the tag.
    #[serde(default)]
    pub key: String,
    /// Display label for the tag.
    #[serde(default)]
    pub tag: String,
}

impl TeamTagItem {
    pub fn builder() -> TeamTagItemBuilder {
        <TeamTagItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamTagItemBuilder {
    key: Option<String>,
    tag: Option<String>,
}

impl TeamTagItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TeamTagItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](TeamTagItemBuilder::key)
    /// - [`tag`](TeamTagItemBuilder::tag)
    pub fn build(self) -> Result<TeamTagItem, BuildError> {
        Ok(TeamTagItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            tag: self.tag.ok_or_else(|| BuildError::missing_field("tag"))?,
        })
    }
}
