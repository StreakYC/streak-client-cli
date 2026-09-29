pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PipelinePermissionSetInput {
    /// Permissions granted by this set.
    #[serde(default)]
    pub permissions: Vec<PipelinePermissionInput>,
    /// Whether users may be assigned this permission set.
    #[serde(rename = "userAssignable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_assignable: Option<bool>,
}

impl PipelinePermissionSetInput {
    pub fn builder() -> PipelinePermissionSetInputBuilder {
        <PipelinePermissionSetInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelinePermissionSetInputBuilder {
    permissions: Option<Vec<PipelinePermissionInput>>,
    user_assignable: Option<bool>,
}

impl PipelinePermissionSetInputBuilder {
    pub fn permissions(mut self, value: Vec<PipelinePermissionInput>) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn user_assignable(mut self, value: bool) -> Self {
        self.user_assignable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelinePermissionSetInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`permissions`](PipelinePermissionSetInputBuilder::permissions)
    pub fn build(self) -> Result<PipelinePermissionSetInput, BuildError> {
        Ok(PipelinePermissionSetInput {
            permissions: self.permissions.ok_or_else(|| BuildError::missing_field("permissions"))?,
            user_assignable: self.user_assignable,
        })
    }
}
