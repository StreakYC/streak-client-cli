pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for getTasksForBox
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTasksForBoxQueryRequest {
    /// Number of items to return. Maximum: 1000
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Zero-based page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl GetTasksForBoxQueryRequest {
    pub fn builder() -> GetTasksForBoxQueryRequestBuilder {
        <GetTasksForBoxQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTasksForBoxQueryRequestBuilder {
    limit: Option<i64>,
    page: Option<i64>,
}

impl GetTasksForBoxQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetTasksForBoxQueryRequest`].
    pub fn build(self) -> Result<GetTasksForBoxQueryRequest, BuildError> {
        Ok(GetTasksForBoxQueryRequest {
            limit: self.limit,
            page: self.page,
        })
    }
}

