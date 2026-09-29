pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A custom contact or organization field definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TeamField {
    /// Display name for the field.
    #[serde(default)]
    pub name: String,
    /// Stable key for the field.
    #[serde(default)]
    pub key: String,
    /// Data type stored by the field.
    pub r#type: FieldType,
    /// Default textual field value, when one is configured.
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// Timestamp when the default value last changed, in epoch milliseconds.
    #[serde(rename = "lastDefaultValueUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_default_value_updated_timestamp: Option<i64>,
    /// Timestamp when the field definition last changed, in epoch milliseconds.
    #[serde(rename = "lastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<i64>,
    /// Allowed dropdown options for a dropdown field.
    #[serde(rename = "dropdownSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown_settings: Option<TeamDropdownSettings>,
    /// Allowed tags for a tag field.
    #[serde(rename = "tagSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_settings: Option<TeamTagSettings>,
}

impl TeamField {
    pub fn builder() -> TeamFieldBuilder {
        <TeamFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamFieldBuilder {
    name: Option<String>,
    key: Option<String>,
    r#type: Option<FieldType>,
    default_value: Option<String>,
    last_default_value_updated_timestamp: Option<i64>,
    last_updated_timestamp: Option<i64>,
    dropdown_settings: Option<TeamDropdownSettings>,
    tag_settings: Option<TeamTagSettings>,
}

impl TeamFieldBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
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

    pub fn last_default_value_updated_timestamp(mut self, value: i64) -> Self {
        self.last_default_value_updated_timestamp = Some(value);
        self
    }

    pub fn last_updated_timestamp(mut self, value: i64) -> Self {
        self.last_updated_timestamp = Some(value);
        self
    }

    pub fn dropdown_settings(mut self, value: TeamDropdownSettings) -> Self {
        self.dropdown_settings = Some(value);
        self
    }

    pub fn tag_settings(mut self, value: TeamTagSettings) -> Self {
        self.tag_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamField`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](TeamFieldBuilder::name)
    /// - [`key`](TeamFieldBuilder::key)
    /// - [`r#type`](TeamFieldBuilder::r#type)
    pub fn build(self) -> Result<TeamField, BuildError> {
        Ok(TeamField {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            default_value: self.default_value,
            last_default_value_updated_timestamp: self.last_default_value_updated_timestamp,
            last_updated_timestamp: self.last_updated_timestamp,
            dropdown_settings: self.dropdown_settings,
            tag_settings: self.tag_settings,
        })
    }
}
