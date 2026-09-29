pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineStage {
    /// Stage key within the pipeline.
    #[serde(default)]
    pub key: String,
    /// Stage name.
    #[serde(default)]
    pub name: String,
    /// Stage color.
    #[serde(default)]
    pub color: PipelineStageColor,
    /// Whether the color is inherited from the pipeline theme.
    #[serde(rename = "isColorFromTheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_color_from_theme: Option<bool>,
    /// Number of boxes currently in this stage.
    #[serde(rename = "boxCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_count: Option<i64>,
}

impl PipelineStage {
    pub fn builder() -> PipelineStageBuilder {
        <PipelineStageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineStageBuilder {
    key: Option<String>,
    name: Option<String>,
    color: Option<PipelineStageColor>,
    is_color_from_theme: Option<bool>,
    box_count: Option<i64>,
}

impl PipelineStageBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn color(mut self, value: PipelineStageColor) -> Self {
        self.color = Some(value);
        self
    }

    pub fn is_color_from_theme(mut self, value: bool) -> Self {
        self.is_color_from_theme = Some(value);
        self
    }

    pub fn box_count(mut self, value: i64) -> Self {
        self.box_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineStage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PipelineStageBuilder::key)
    /// - [`name`](PipelineStageBuilder::name)
    /// - [`color`](PipelineStageBuilder::color)
    pub fn build(self) -> Result<PipelineStage, BuildError> {
        Ok(PipelineStage {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            color: self.color.ok_or_else(|| BuildError::missing_field("color"))?,
            is_color_from_theme: self.is_color_from_theme,
            box_count: self.box_count,
        })
    }
}
