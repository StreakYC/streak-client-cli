pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for listPipelines
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPipelinesQueryRequest {
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// Number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Zero-based page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl ListPipelinesQueryRequest {
    pub fn builder() -> ListPipelinesQueryRequestBuilder {
        <ListPipelinesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPipelinesQueryRequestBuilder {
    sort_by: Option<String>,
    limit: Option<i64>,
    page: Option<i64>,
}

impl ListPipelinesQueryRequestBuilder {
    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPipelinesQueryRequest`].
    pub fn build(self) -> Result<ListPipelinesQueryRequest, BuildError> {
        Ok(ListPipelinesQueryRequest {
            sort_by: self.sort_by,
            limit: self.limit,
            page: self.page,
        })
    }
}

