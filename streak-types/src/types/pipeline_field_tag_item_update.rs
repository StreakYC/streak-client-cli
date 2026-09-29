pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An existing or new tag option.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineFieldTagItemUpdate {
    /// Existing tag key. Omit the key when adding a tag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Nonempty tag label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl PipelineFieldTagItemUpdate {
    pub fn builder() -> PipelineFieldTagItemUpdateBuilder {
        <PipelineFieldTagItemUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineFieldTagItemUpdateBuilder {
    key: Option<String>,
    tag: Option<String>,
}

impl PipelineFieldTagItemUpdateBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PipelineFieldTagItemUpdate`].
    pub fn build(self) -> Result<PipelineFieldTagItemUpdate, BuildError> {
        Ok(PipelineFieldTagItemUpdate {
            key: self.key,
            tag: self.tag,
        })
    }
}
