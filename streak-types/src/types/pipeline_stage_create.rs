pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineStageCreate {
    /// Nonempty name of the new stage.
    #[serde(default)]
    pub name: String,
    /// Custom stage colors. Omit or set to null to use the pipeline theme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<PipelineStageColor>,
}

impl PipelineStageCreate {
    pub fn builder() -> PipelineStageCreateBuilder {
        <PipelineStageCreateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineStageCreateBuilder {
    name: Option<String>,
    color: Option<PipelineStageColor>,
}

impl PipelineStageCreateBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn color(mut self, value: PipelineStageColor) -> Self {
        self.color = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineStageCreate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PipelineStageCreateBuilder::name)
    pub fn build(self) -> Result<PipelineStageCreate, BuildError> {
        Ok(PipelineStageCreate {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            color: self.color,
        })
    }
}

