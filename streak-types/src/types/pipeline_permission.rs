pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PipelinePermission {
    /// Entity type governed by the permission.
    #[serde(rename = "entityType")]
    #[serde(default)]
    pub entity_type: String,
    /// Action granted by the permission.
    pub action: Action,
    /// Condition that must be satisfied for the permission to apply.
    #[serde(rename = "conditionalCheck")]
    pub conditional_check: PipelineConditionalCheck,
    /// Additional parameters used by the conditional check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<HashMap<String, Option<String>>>,
}

impl PipelinePermission {
    pub fn builder() -> PipelinePermissionBuilder {
        <PipelinePermissionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelinePermissionBuilder {
    entity_type: Option<String>,
    action: Option<Action>,
    conditional_check: Option<PipelineConditionalCheck>,
    params: Option<HashMap<String, Option<String>>>,
}

impl PipelinePermissionBuilder {
    pub fn entity_type(mut self, value: impl Into<String>) -> Self {
        self.entity_type = Some(value.into());
        self
    }

    pub fn action(mut self, value: Action) -> Self {
        self.action = Some(value);
        self
    }

    pub fn conditional_check(mut self, value: PipelineConditionalCheck) -> Self {
        self.conditional_check = Some(value);
        self
    }

    pub fn params(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.params = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelinePermission`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entity_type`](PipelinePermissionBuilder::entity_type)
    /// - [`action`](PipelinePermissionBuilder::action)
    /// - [`conditional_check`](PipelinePermissionBuilder::conditional_check)
    pub fn build(self) -> Result<PipelinePermission, BuildError> {
        Ok(PipelinePermission {
            entity_type: self.entity_type.ok_or_else(|| BuildError::missing_field("entity_type"))?,
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            conditional_check: self.conditional_check.ok_or_else(|| BuildError::missing_field("conditional_check"))?,
            params: self.params,
        })
    }
}
