pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PipelineUpdate {
    /// New team key. Omit to keep the current team.
    #[serde(rename = "teamKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_key: Option<String>,
    /// New icon. Omit to keep the current icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// New name. Omit to keep the current name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New description. Omit to keep the current description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// New team sharing restriction. Omit to keep the current value.
    #[serde(rename = "sharingRestrictedToTeam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_restricted_to_team: Option<bool>,
    /// Complete stage order. Omit to keep the current order; an empty list is invalid.
    #[serde(rename = "stageOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_order: Option<Vec<String>>,
    /// New stage color theme. Omit to keep the current theme.
    #[serde(rename = "stageColorTheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_color_theme: Option<String>,
    /// Replacement custom permission sets. Omit to keep existing sets.
    #[serde(rename = "customPermissionSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_permission_sets: Option<HashMap<String, Option<PipelinePermissionSetInput>>>,
    /// New default permission set. Omit to keep the current value.
    #[serde(rename = "defaultPermissionSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission_set_name: Option<String>,
    /// Replacement sharing entries. Omit to keep existing sharing; an empty list is invalid.
    #[serde(rename = "aclEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl_entries: Option<Vec<PipelineSharingEntryInput>>,
}

impl PipelineUpdate {
    pub fn builder() -> PipelineUpdateBuilder {
        <PipelineUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineUpdateBuilder {
    team_key: Option<String>,
    icon: Option<String>,
    name: Option<String>,
    description: Option<String>,
    sharing_restricted_to_team: Option<bool>,
    stage_order: Option<Vec<String>>,
    stage_color_theme: Option<String>,
    custom_permission_sets: Option<HashMap<String, Option<PipelinePermissionSetInput>>>,
    default_permission_set_name: Option<String>,
    acl_entries: Option<Vec<PipelineSharingEntryInput>>,
}

impl PipelineUpdateBuilder {
    pub fn team_key(mut self, value: impl Into<String>) -> Self {
        self.team_key = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn sharing_restricted_to_team(mut self, value: bool) -> Self {
        self.sharing_restricted_to_team = Some(value);
        self
    }

    pub fn stage_order(mut self, value: Vec<String>) -> Self {
        self.stage_order = Some(value);
        self
    }

    pub fn stage_color_theme(mut self, value: impl Into<String>) -> Self {
        self.stage_color_theme = Some(value.into());
        self
    }

    pub fn custom_permission_sets(mut self, value: HashMap<String, Option<PipelinePermissionSetInput>>) -> Self {
        self.custom_permission_sets = Some(value);
        self
    }

    pub fn default_permission_set_name(mut self, value: impl Into<String>) -> Self {
        self.default_permission_set_name = Some(value.into());
        self
    }

    pub fn acl_entries(mut self, value: Vec<PipelineSharingEntryInput>) -> Self {
        self.acl_entries = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineUpdate`].
    pub fn build(self) -> Result<PipelineUpdate, BuildError> {
        Ok(PipelineUpdate {
            team_key: self.team_key,
            icon: self.icon,
            name: self.name,
            description: self.description,
            sharing_restricted_to_team: self.sharing_restricted_to_team,
            stage_order: self.stage_order,
            stage_color_theme: self.stage_color_theme,
            custom_permission_sets: self.custom_permission_sets,
            default_permission_set_name: self.default_permission_set_name,
            acl_entries: self.acl_entries,
        })
    }
}

