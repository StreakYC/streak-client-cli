pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineDropdownSettings {
    /// Available dropdown options.
    #[serde(default)]
    pub items: Vec<PipelineDropdownItem>,
}

impl PipelineDropdownSettings {
    pub fn builder() -> PipelineDropdownSettingsBuilder {
        <PipelineDropdownSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineDropdownSettingsBuilder {
    items: Option<Vec<PipelineDropdownItem>>,
}

impl PipelineDropdownSettingsBuilder {
    pub fn items(mut self, value: Vec<PipelineDropdownItem>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineDropdownSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](PipelineDropdownSettingsBuilder::items)
    pub fn build(self) -> Result<PipelineDropdownSettings, BuildError> {
        Ok(PipelineDropdownSettings {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
