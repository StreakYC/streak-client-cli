pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineStageColor {
    /// Foreground color as a CSS color string.
    #[serde(rename = "foregroundColor")]
    #[serde(default)]
    pub foreground_color: String,
    /// Background color as a CSS color string.
    #[serde(rename = "backgroundColor")]
    #[serde(default)]
    pub background_color: String,
}

impl PipelineStageColor {
    pub fn builder() -> PipelineStageColorBuilder {
        <PipelineStageColorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineStageColorBuilder {
    foreground_color: Option<String>,
    background_color: Option<String>,
}

impl PipelineStageColorBuilder {
    pub fn foreground_color(mut self, value: impl Into<String>) -> Self {
        self.foreground_color = Some(value.into());
        self
    }

    pub fn background_color(mut self, value: impl Into<String>) -> Self {
        self.background_color = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PipelineStageColor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`foreground_color`](PipelineStageColorBuilder::foreground_color)
    /// - [`background_color`](PipelineStageColorBuilder::background_color)
    pub fn build(self) -> Result<PipelineStageColor, BuildError> {
        Ok(PipelineStageColor {
            foreground_color: self.foreground_color.ok_or_else(|| BuildError::missing_field("foreground_color"))?,
            background_color: self.background_color.ok_or_else(|| BuildError::missing_field("background_color"))?,
        })
    }
}
