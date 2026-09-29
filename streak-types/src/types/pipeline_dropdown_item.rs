pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineDropdownItem {
    /// Option key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Option name.
    #[serde(default)]
    pub name: String,
}

impl PipelineDropdownItem {
    pub fn builder() -> PipelineDropdownItemBuilder {
        <PipelineDropdownItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineDropdownItemBuilder {
    key: Option<String>,
    name: Option<String>,
}

impl PipelineDropdownItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PipelineDropdownItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PipelineDropdownItemBuilder::name)
    pub fn build(self) -> Result<PipelineDropdownItem, BuildError> {
        Ok(PipelineDropdownItem {
            key: self.key,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
