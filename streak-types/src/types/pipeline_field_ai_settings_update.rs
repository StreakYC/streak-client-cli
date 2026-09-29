pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Changes to AI settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineFieldAiSettingsUpdate {
    /// AI prompt. An empty string clears it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Content used for AI autofill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl PipelineFieldAiSettingsUpdate {
    pub fn builder() -> PipelineFieldAiSettingsUpdateBuilder {
        <PipelineFieldAiSettingsUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineFieldAiSettingsUpdateBuilder {
    prompt: Option<String>,
    source: Option<String>,
}

impl PipelineFieldAiSettingsUpdateBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PipelineFieldAiSettingsUpdate`].
    pub fn build(self) -> Result<PipelineFieldAiSettingsUpdate, BuildError> {
        Ok(PipelineFieldAiSettingsUpdate {
            prompt: self.prompt,
            source: self.source,
        })
    }
}
