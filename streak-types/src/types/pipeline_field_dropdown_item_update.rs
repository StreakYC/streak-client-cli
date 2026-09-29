pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An existing or new dropdown option.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineFieldDropdownItemUpdate {
    /// Existing option key. Omit the key when adding an option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Nonempty option label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PipelineFieldDropdownItemUpdate {
    pub fn builder() -> PipelineFieldDropdownItemUpdateBuilder {
        <PipelineFieldDropdownItemUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineFieldDropdownItemUpdateBuilder {
    key: Option<String>,
    name: Option<String>,
}

impl PipelineFieldDropdownItemUpdateBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PipelineFieldDropdownItemUpdate`].
    pub fn build(self) -> Result<PipelineFieldDropdownItemUpdate, BuildError> {
        Ok(PipelineFieldDropdownItemUpdate {
            key: self.key,
            name: self.name,
        })
    }
}
