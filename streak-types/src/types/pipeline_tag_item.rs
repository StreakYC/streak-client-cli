pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineTagItem {
    /// Tag key.
    #[serde(default)]
    pub key: String,
    /// Tag label.
    #[serde(default)]
    pub tag: String,
}

impl PipelineTagItem {
    pub fn builder() -> PipelineTagItemBuilder {
        <PipelineTagItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineTagItemBuilder {
    key: Option<String>,
    tag: Option<String>,
}

impl PipelineTagItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PipelineTagItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PipelineTagItemBuilder::key)
    /// - [`tag`](PipelineTagItemBuilder::tag)
    pub fn build(self) -> Result<PipelineTagItem, BuildError> {
        Ok(PipelineTagItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            tag: self.tag.ok_or_else(|| BuildError::missing_field("tag"))?,
        })
    }
}
