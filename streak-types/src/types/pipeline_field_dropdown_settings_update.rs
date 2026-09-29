pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Dropdown options in the requested order.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineFieldDropdownSettingsUpdate {
    /// Options to keep or add. An empty list removes all options unless addOnly is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PipelineFieldDropdownItemUpdate>>,
}

impl PipelineFieldDropdownSettingsUpdate {
    pub fn builder() -> PipelineFieldDropdownSettingsUpdateBuilder {
        <PipelineFieldDropdownSettingsUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineFieldDropdownSettingsUpdateBuilder {
    items: Option<Vec<PipelineFieldDropdownItemUpdate>>,
}

impl PipelineFieldDropdownSettingsUpdateBuilder {
    pub fn items(mut self, value: Vec<PipelineFieldDropdownItemUpdate>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineFieldDropdownSettingsUpdate`].
    pub fn build(self) -> Result<PipelineFieldDropdownSettingsUpdate, BuildError> {
        Ok(PipelineFieldDropdownSettingsUpdate {
            items: self.items,
        })
    }
}
