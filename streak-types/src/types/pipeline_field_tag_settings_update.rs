pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Tag options in the requested order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineFieldTagSettingsUpdate {
    /// Tags to keep or add. An empty list removes all tags unless addOnly is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<PipelineFieldTagItemUpdate>>,
}

impl PipelineFieldTagSettingsUpdate {
    pub fn builder() -> PipelineFieldTagSettingsUpdateBuilder {
        <PipelineFieldTagSettingsUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineFieldTagSettingsUpdateBuilder {
    tags: Option<Vec<PipelineFieldTagItemUpdate>>,
}

impl PipelineFieldTagSettingsUpdateBuilder {
    pub fn tags(mut self, value: Vec<PipelineFieldTagItemUpdate>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineFieldTagSettingsUpdate`].
    pub fn build(self) -> Result<PipelineFieldTagSettingsUpdate, BuildError> {
        Ok(PipelineFieldTagSettingsUpdate {
            tags: self.tags,
        })
    }
}
