pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineFieldUpdate {
    /// New field name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New default value or formula.
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// Dropdown options to apply.
    #[serde(rename = "dropdownSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown_settings: Option<PipelineFieldDropdownSettingsUpdate>,
    /// Tag options to apply.
    #[serde(rename = "tagSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_settings: Option<PipelineFieldTagSettingsUpdate>,
    /// AI settings to update.
    #[serde(rename = "aiSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_settings: Option<PipelineFieldAiSettingsUpdate>,
    /// Append dropdown or tag options while retaining existing options. Defaults to false.
    #[serde(rename = "addOnly")]
    #[serde(skip)]
    pub add_only: Option<bool>,
}

impl PipelineFieldUpdate {
    pub fn builder() -> PipelineFieldUpdateBuilder {
        <PipelineFieldUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineFieldUpdateBuilder {
    name: Option<String>,
    default_value: Option<String>,
    dropdown_settings: Option<PipelineFieldDropdownSettingsUpdate>,
    tag_settings: Option<PipelineFieldTagSettingsUpdate>,
    ai_settings: Option<PipelineFieldAiSettingsUpdate>,
    add_only: Option<bool>,
}

impl PipelineFieldUpdateBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn dropdown_settings(mut self, value: PipelineFieldDropdownSettingsUpdate) -> Self {
        self.dropdown_settings = Some(value);
        self
    }

    pub fn tag_settings(mut self, value: PipelineFieldTagSettingsUpdate) -> Self {
        self.tag_settings = Some(value);
        self
    }

    pub fn ai_settings(mut self, value: PipelineFieldAiSettingsUpdate) -> Self {
        self.ai_settings = Some(value);
        self
    }

    pub fn add_only(mut self, value: bool) -> Self {
        self.add_only = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineFieldUpdate`].
    pub fn build(self) -> Result<PipelineFieldUpdate, BuildError> {
        Ok(PipelineFieldUpdate {
            name: self.name,
            default_value: self.default_value,
            dropdown_settings: self.dropdown_settings,
            tag_settings: self.tag_settings,
            ai_settings: self.ai_settings,
            add_only: self.add_only,
        })
    }
}

