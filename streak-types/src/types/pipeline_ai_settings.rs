pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PipelineAiSettings {
    /// Prompt used for AI autofill.
    #[serde(default)]
    pub prompt: String,
    /// Content used as context for AI autofill.
    pub source: PipelineAutofillSource,
}

impl PipelineAiSettings {
    pub fn builder() -> PipelineAiSettingsBuilder {
        <PipelineAiSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineAiSettingsBuilder {
    prompt: Option<String>,
    source: Option<PipelineAutofillSource>,
}

impl PipelineAiSettingsBuilder {
    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn source(mut self, value: PipelineAutofillSource) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineAiSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt`](PipelineAiSettingsBuilder::prompt)
    /// - [`source`](PipelineAiSettingsBuilder::source)
    pub fn build(self) -> Result<PipelineAiSettings, BuildError> {
        Ok(PipelineAiSettings {
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}
