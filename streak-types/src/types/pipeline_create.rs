pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PipelineCreate {
    /// Pipeline name.
    #[serde(default)]
    pub name: String,
    /// Key for the team that will own the pipeline.
    #[serde(rename = "teamKey")]
    #[serde(default)]
    pub team_key: String,
    /// Stage names in display order.
    #[serde(default)]
    pub stages: Vec<String>,
    /// Pipeline icon name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// Pipeline description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Template identifier associated with the pipeline.
    #[serde(rename = "templateType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
    /// Fields to create in the pipeline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<PipelineFieldCreate>>,
    /// Default permission set for users without an explicit assignment.
    #[serde(rename = "defaultPermissionSetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission_set_name: Option<String>,
    /// Whether sharing is restricted to members of the owning team.
    #[serde(rename = "sharingRestrictedToTeam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_restricted_to_team: Option<bool>,
}

impl PipelineCreate {
    pub fn builder() -> PipelineCreateBuilder {
        <PipelineCreateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineCreateBuilder {
    name: Option<String>,
    team_key: Option<String>,
    stages: Option<Vec<String>>,
    icon: Option<String>,
    description: Option<String>,
    template_type: Option<String>,
    fields: Option<Vec<PipelineFieldCreate>>,
    default_permission_set_name: Option<String>,
    sharing_restricted_to_team: Option<bool>,
}

impl PipelineCreateBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn team_key(mut self, value: impl Into<String>) -> Self {
        self.team_key = Some(value.into());
        self
    }

    pub fn stages(mut self, value: Vec<String>) -> Self {
        self.stages = Some(value);
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

    pub fn template_type(mut self, value: impl Into<String>) -> Self {
        self.template_type = Some(value.into());
        self
    }

    pub fn fields(mut self, value: Vec<PipelineFieldCreate>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn default_permission_set_name(mut self, value: impl Into<String>) -> Self {
        self.default_permission_set_name = Some(value.into());
        self
    }

    pub fn sharing_restricted_to_team(mut self, value: bool) -> Self {
        self.sharing_restricted_to_team = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineCreate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](PipelineCreateBuilder::name)
    /// - [`team_key`](PipelineCreateBuilder::team_key)
    /// - [`stages`](PipelineCreateBuilder::stages)
    pub fn build(self) -> Result<PipelineCreate, BuildError> {
        Ok(PipelineCreate {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            team_key: self.team_key.ok_or_else(|| BuildError::missing_field("team_key"))?,
            stages: self.stages.ok_or_else(|| BuildError::missing_field("stages"))?,
            icon: self.icon,
            description: self.description,
            template_type: self.template_type,
            fields: self.fields,
            default_permission_set_name: self.default_permission_set_name,
            sharing_restricted_to_team: self.sharing_restricted_to_team,
        })
    }
}

