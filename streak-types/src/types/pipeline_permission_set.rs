pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PipelinePermissionSet {
    /// Permissions granted by this set.
    #[serde(default)]
    pub permissions: Vec<PipelinePermission>,
    /// Whether users may be assigned this permission set.
    #[serde(rename = "userAssignable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_assignable: Option<bool>,
}

impl PipelinePermissionSet {
    pub fn builder() -> PipelinePermissionSetBuilder {
        <PipelinePermissionSetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelinePermissionSetBuilder {
    permissions: Option<Vec<PipelinePermission>>,
    user_assignable: Option<bool>,
}

impl PipelinePermissionSetBuilder {
    pub fn permissions(mut self, value: Vec<PipelinePermission>) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn user_assignable(mut self, value: bool) -> Self {
        self.user_assignable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelinePermissionSet`].
    /// This method will fail if any of the following fields are not set:
    /// - [`permissions`](PipelinePermissionSetBuilder::permissions)
    pub fn build(self) -> Result<PipelinePermissionSet, BuildError> {
        Ok(PipelinePermissionSet {
            permissions: self.permissions.ok_or_else(|| BuildError::missing_field("permissions"))?,
            user_assignable: self.user_assignable,
        })
    }
}
