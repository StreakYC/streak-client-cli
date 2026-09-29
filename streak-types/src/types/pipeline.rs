pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Pipeline {
    /// Key for this pipeline.
    #[serde(default)]
    pub key: String,
    /// Key for the team that owns this pipeline.
    #[serde(rename = "teamKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_key: Option<String>,
    /// Key for the user who created this pipeline.
    #[serde(rename = "creatorKey")]
    #[serde(default)]
    pub creator_key: String,
    /// Pipeline name.
    #[serde(default)]
    pub name: String,
    /// Pipeline icon name.
    #[serde(default)]
    pub icon: String,
    /// Pipeline description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether sharing is restricted to the team.
    #[serde(rename = "sharingRestrictedToTeam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_restricted_to_team: Option<bool>,
    /// Custom fields configured for this pipeline.
    #[serde(default)]
    pub fields: Vec<PipelineField>,
    /// Stages in this pipeline, keyed by stage key.
    #[serde(default)]
    pub stages: HashMap<String, PipelineStage>,
    /// The display order of the stages
    #[serde(rename = "stageOrder")]
    #[serde(default)]
    pub stage_order: Vec<String>,
    /// Color theme used for stages without custom colors.
    #[serde(rename = "stageColorTheme")]
    pub stage_color_theme: WorkflowTheme,
    /// Timestamp when the pipeline was created, in epoch milliseconds.
    #[serde(rename = "creationTimestamp")]
    #[serde(default)]
    pub creation_timestamp: i64,
    /// Timestamp when the pipeline was last updated, in epoch milliseconds.
    #[serde(rename = "lastUpdatedTimestamp")]
    #[serde(default)]
    pub last_updated_timestamp: i64,
    /// Latest formula-definition update timestamp, in epoch milliseconds.
    #[serde(rename = "formulasLastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formulas_last_updated_timestamp: Option<i64>,
    /// Custom permission sets configured for this pipeline.
    #[serde(rename = "customPermissionSets")]
    #[serde(default)]
    pub custom_permission_sets: HashMap<String, PipelinePermissionSet>,
    /// Default permission set assigned to users without an explicit assignment.
    #[serde(rename = "defaultPermissionSetName")]
    #[serde(default)]
    pub default_permission_set_name: String,
    /// Users explicitly sharing this pipeline and their permission assignments.
    #[serde(rename = "aclEntries")]
    #[serde(default)]
    pub acl_entries: Vec<PipelineSharingEntry>,
    /// Source that created the pipeline.
    #[serde(rename = "creationSourceType")]
    pub creation_source_type: ActivitySourceType,
    /// Current number of boxes in the pipeline.
    #[serde(rename = "boxCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_count: Option<i64>,
    /// Timestamp when the pipeline was last persisted, in epoch milliseconds.
    #[serde(rename = "lastSavedTimestamp")]
    #[serde(default)]
    pub last_saved_timestamp: i64,
}

impl Pipeline {
    pub fn builder() -> PipelineBuilder {
        <PipelineBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineBuilder {
    key: Option<String>,
    team_key: Option<String>,
    creator_key: Option<String>,
    name: Option<String>,
    icon: Option<String>,
    description: Option<String>,
    sharing_restricted_to_team: Option<bool>,
    fields: Option<Vec<PipelineField>>,
    stages: Option<HashMap<String, PipelineStage>>,
    stage_order: Option<Vec<String>>,
    stage_color_theme: Option<WorkflowTheme>,
    creation_timestamp: Option<i64>,
    last_updated_timestamp: Option<i64>,
    formulas_last_updated_timestamp: Option<i64>,
    custom_permission_sets: Option<HashMap<String, PipelinePermissionSet>>,
    default_permission_set_name: Option<String>,
    acl_entries: Option<Vec<PipelineSharingEntry>>,
    creation_source_type: Option<ActivitySourceType>,
    box_count: Option<i64>,
    last_saved_timestamp: Option<i64>,
}

impl PipelineBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn team_key(mut self, value: impl Into<String>) -> Self {
        self.team_key = Some(value.into());
        self
    }

    pub fn creator_key(mut self, value: impl Into<String>) -> Self {
        self.creator_key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
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

    pub fn fields(mut self, value: Vec<PipelineField>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn stages(mut self, value: HashMap<String, PipelineStage>) -> Self {
        self.stages = Some(value);
        self
    }

    pub fn stage_order(mut self, value: Vec<String>) -> Self {
        self.stage_order = Some(value);
        self
    }

    pub fn stage_color_theme(mut self, value: WorkflowTheme) -> Self {
        self.stage_color_theme = Some(value);
        self
    }

    pub fn creation_timestamp(mut self, value: i64) -> Self {
        self.creation_timestamp = Some(value);
        self
    }

    pub fn last_updated_timestamp(mut self, value: i64) -> Self {
        self.last_updated_timestamp = Some(value);
        self
    }

    pub fn formulas_last_updated_timestamp(mut self, value: i64) -> Self {
        self.formulas_last_updated_timestamp = Some(value);
        self
    }

    pub fn custom_permission_sets(mut self, value: HashMap<String, PipelinePermissionSet>) -> Self {
        self.custom_permission_sets = Some(value);
        self
    }

    pub fn default_permission_set_name(mut self, value: impl Into<String>) -> Self {
        self.default_permission_set_name = Some(value.into());
        self
    }

    pub fn acl_entries(mut self, value: Vec<PipelineSharingEntry>) -> Self {
        self.acl_entries = Some(value);
        self
    }

    pub fn creation_source_type(mut self, value: ActivitySourceType) -> Self {
        self.creation_source_type = Some(value);
        self
    }

    pub fn box_count(mut self, value: i64) -> Self {
        self.box_count = Some(value);
        self
    }

    pub fn last_saved_timestamp(mut self, value: i64) -> Self {
        self.last_saved_timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Pipeline`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](PipelineBuilder::key)
    /// - [`creator_key`](PipelineBuilder::creator_key)
    /// - [`name`](PipelineBuilder::name)
    /// - [`icon`](PipelineBuilder::icon)
    /// - [`fields`](PipelineBuilder::fields)
    /// - [`stages`](PipelineBuilder::stages)
    /// - [`stage_order`](PipelineBuilder::stage_order)
    /// - [`stage_color_theme`](PipelineBuilder::stage_color_theme)
    /// - [`creation_timestamp`](PipelineBuilder::creation_timestamp)
    /// - [`last_updated_timestamp`](PipelineBuilder::last_updated_timestamp)
    /// - [`custom_permission_sets`](PipelineBuilder::custom_permission_sets)
    /// - [`default_permission_set_name`](PipelineBuilder::default_permission_set_name)
    /// - [`acl_entries`](PipelineBuilder::acl_entries)
    /// - [`creation_source_type`](PipelineBuilder::creation_source_type)
    /// - [`last_saved_timestamp`](PipelineBuilder::last_saved_timestamp)
    pub fn build(self) -> Result<Pipeline, BuildError> {
        Ok(Pipeline {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            team_key: self.team_key,
            creator_key: self.creator_key.ok_or_else(|| BuildError::missing_field("creator_key"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            icon: self.icon.ok_or_else(|| BuildError::missing_field("icon"))?,
            description: self.description,
            sharing_restricted_to_team: self.sharing_restricted_to_team,
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            stages: self.stages.ok_or_else(|| BuildError::missing_field("stages"))?,
            stage_order: self.stage_order.ok_or_else(|| BuildError::missing_field("stage_order"))?,
            stage_color_theme: self.stage_color_theme.ok_or_else(|| BuildError::missing_field("stage_color_theme"))?,
            creation_timestamp: self.creation_timestamp.ok_or_else(|| BuildError::missing_field("creation_timestamp"))?,
            last_updated_timestamp: self.last_updated_timestamp.ok_or_else(|| BuildError::missing_field("last_updated_timestamp"))?,
            formulas_last_updated_timestamp: self.formulas_last_updated_timestamp,
            custom_permission_sets: self.custom_permission_sets.ok_or_else(|| BuildError::missing_field("custom_permission_sets"))?,
            default_permission_set_name: self.default_permission_set_name.ok_or_else(|| BuildError::missing_field("default_permission_set_name"))?,
            acl_entries: self.acl_entries.ok_or_else(|| BuildError::missing_field("acl_entries"))?,
            creation_source_type: self.creation_source_type.ok_or_else(|| BuildError::missing_field("creation_source_type"))?,
            box_count: self.box_count,
            last_saved_timestamp: self.last_saved_timestamp.ok_or_else(|| BuildError::missing_field("last_saved_timestamp"))?,
        })
    }
}
