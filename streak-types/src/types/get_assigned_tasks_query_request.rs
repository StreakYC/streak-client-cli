pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for getAssignedTasks
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAssignedTasksQueryRequest {
    /// Agenda direction. Accepted values are desc, past, asc, and future. Defaults to desc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// Whether completed tasks should be included in the returned agenda.
    #[serde(rename = "includeCompleted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_completed: Option<bool>,
    /// Maximum number of tasks to return per assigned user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Key for the pipeline to filter tasks to. Omit to search across all visible pipelines.
    #[serde(rename = "pipelineKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_key: Option<String>,
    /// Opaque 30-digit sort cursor returned as task.sortOrder. Omit to use tomorrow at local midnight as the cursor boundary.
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// User keys whose assigned tasks should be returned. Omit to return tasks assigned to the current user.
    #[serde(rename = "userKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_keys: Option<Vec<String>>,
}

impl GetAssignedTasksQueryRequest {
    pub fn builder() -> GetAssignedTasksQueryRequestBuilder {
        <GetAssignedTasksQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAssignedTasksQueryRequestBuilder {
    direction: Option<String>,
    include_completed: Option<bool>,
    limit: Option<i64>,
    pipeline_key: Option<String>,
    sort_order: Option<String>,
    user_keys: Option<Vec<String>>,
}

impl GetAssignedTasksQueryRequestBuilder {
    pub fn direction(mut self, value: impl Into<String>) -> Self {
        self.direction = Some(value.into());
        self
    }

    pub fn include_completed(mut self, value: bool) -> Self {
        self.include_completed = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn pipeline_key(mut self, value: impl Into<String>) -> Self {
        self.pipeline_key = Some(value.into());
        self
    }

    pub fn sort_order(mut self, value: impl Into<String>) -> Self {
        self.sort_order = Some(value.into());
        self
    }

    pub fn user_keys(mut self, value: Vec<String>) -> Self {
        self.user_keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAssignedTasksQueryRequest`].
    pub fn build(self) -> Result<GetAssignedTasksQueryRequest, BuildError> {
        Ok(GetAssignedTasksQueryRequest {
            direction: self.direction,
            include_completed: self.include_completed,
            limit: self.limit,
            pipeline_key: self.pipeline_key,
            sort_order: self.sort_order,
            user_keys: self.user_keys,
        })
    }
}

