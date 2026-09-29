pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PipelineField {
    /// Field key within the pipeline.
    #[serde(default)]
    pub key: String,
    /// Field name.
    #[serde(default)]
    pub name: String,
    /// Field data type.
    pub r#type: FieldType,
    /// Default value for this field.
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
    /// Timestamp when the default value last changed, in epoch milliseconds.
    #[serde(rename = "lastDefaultValueUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_default_value_updated_timestamp: Option<i64>,
    /// Timestamp when this field last changed, in epoch milliseconds.
    #[serde(rename = "lastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<i64>,
    /// Dropdown options when this is a dropdown field.
    #[serde(rename = "dropdownSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown_settings: Option<PipelineDropdownSettings>,
    /// Tag options when this is a tag field.
    #[serde(rename = "tagSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_settings: Option<PipelineTagSettings>,
    /// AI autofill settings for this field.
    #[serde(rename = "aiSettings")]
    pub ai_settings: PipelineAiSettings,
}

impl PipelineField {
    pub fn builder() -> PipelineFieldBuilder {
        <PipelineFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineFieldBuilder {
    key: Option<String>,
    name: Option<String>,
    r#type: Option<FieldType>,
    default_value: Option<serde_json::Value>,
    last_default_value_updated_timestamp: Option<i64>,
    last_updated_timestamp: Option<i64>,
    dropdown_settings: Option<PipelineDropdownSettings>,
    tag_settings: Option<PipelineTagSettings>,
    ai_settings: Option<PipelineAiSettings>,
}

impl PipelineFieldBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: FieldType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn default_value(mut self, value: serde_json::Value) -> Self {
        self.default_value = Some(value);
        self
    }

    pub fn last_default_value_updated_timestamp(mut self, value: i64) -> Self {
        self.last_default_value_updated_timestamp = Some(value);
        self
    }

    pub fn last_updated_timestamp(mut self, value: i64) -> Self {
        self.last_updated_timestamp = Some(value);
        self
    }

    pub fn dropdown_settings(mut self, value: PipelineDropdownSettings) -> Self {
        self.dropdown_settings = Some(value);
        self
    }

    pub fn tag_settings(mut self, value: PipelineTagSettings) -> Self {
        self.tag_settings = Some(value);
        self
    }

    pub fn ai_settings(mut self, value: PipelineAiSettings) -> Self {
        self.ai_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineField`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PipelineFieldBuilder::key)
    /// - [`name`](PipelineFieldBuilder::name)
    /// - [`r#type`](PipelineFieldBuilder::r#type)
    /// - [`ai_settings`](PipelineFieldBuilder::ai_settings)
    pub fn build(self) -> Result<PipelineField, BuildError> {
        Ok(PipelineField {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            default_value: self.default_value,
            last_default_value_updated_timestamp: self.last_default_value_updated_timestamp,
            last_updated_timestamp: self.last_updated_timestamp,
            dropdown_settings: self.dropdown_settings,
            tag_settings: self.tag_settings,
            ai_settings: self.ai_settings.ok_or_else(|| BuildError::missing_field("ai_settings"))?,
        })
    }
}
