pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineStageUpdate {
    /// Nonempty new stage name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Custom stage colors. Set both foregroundColor and backgroundColor to empty strings to reset to the pipeline theme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<PipelineStageColor>,
}

impl PipelineStageUpdate {
    pub fn builder() -> PipelineStageUpdateBuilder {
        <PipelineStageUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineStageUpdateBuilder {
    name: Option<String>,
    color: Option<PipelineStageColor>,
}

impl PipelineStageUpdateBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn color(mut self, value: PipelineStageColor) -> Self {
        self.color = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineStageUpdate`].
    pub fn build(self) -> Result<PipelineStageUpdate, BuildError> {
        Ok(PipelineStageUpdate {
            name: self.name,
            color: self.color,
        })
    }
}

