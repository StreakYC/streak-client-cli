pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineTagSettings {
    /// Available tags.
    #[serde(default)]
    pub tags: Vec<PipelineTagItem>,
}

impl PipelineTagSettings {
    pub fn builder() -> PipelineTagSettingsBuilder {
        <PipelineTagSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineTagSettingsBuilder {
    tags: Option<Vec<PipelineTagItem>>,
}

impl PipelineTagSettingsBuilder {
    pub fn tags(mut self, value: Vec<PipelineTagItem>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineTagSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tags`](PipelineTagSettingsBuilder::tags)
    pub fn build(self) -> Result<PipelineTagSettings, BuildError> {
        Ok(PipelineTagSettings {
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
        })
    }
}
