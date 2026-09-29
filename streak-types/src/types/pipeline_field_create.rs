pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PipelineFieldCreate {
    /// Field name.
    #[serde(default)]
    pub name: String,
    /// Field data type.
    pub r#type: FieldType,
    /// Initial default value. Currently only used for formula fields.
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

impl PipelineFieldCreate {
    pub fn builder() -> PipelineFieldCreateBuilder {
        <PipelineFieldCreateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineFieldCreateBuilder {
    name: Option<String>,
    r#type: Option<FieldType>,
    default_value: Option<String>,
}

impl PipelineFieldCreateBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: FieldType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PipelineFieldCreate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PipelineFieldCreateBuilder::name)
    /// - [`r#type`](PipelineFieldCreateBuilder::r#type)
    pub fn build(self) -> Result<PipelineFieldCreate, BuildError> {
        Ok(PipelineFieldCreate {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            default_value: self.default_value,
        })
    }
}
