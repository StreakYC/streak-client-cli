pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A wrapper for a list of Task(s) that supports paging when available.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TaskListResponse {
    /// Whether there are more Task(s) available after this response.
    #[serde(rename = "hasNextPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_next_page: Option<bool>,
    /// The Task(s) returned by this response.
    #[serde(default)]
    pub results: Vec<Task>,
}

impl TaskListResponse {
    pub fn builder() -> TaskListResponseBuilder {
        <TaskListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskListResponseBuilder {
    has_next_page: Option<bool>,
    results: Option<Vec<Task>>,
}

impl TaskListResponseBuilder {
    pub fn has_next_page(mut self, value: bool) -> Self {
        self.has_next_page = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<Task>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](TaskListResponseBuilder::results)
    pub fn build(self) -> Result<TaskListResponse, BuildError> {
        Ok(TaskListResponse {
            has_next_page: self.has_next_page,
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
