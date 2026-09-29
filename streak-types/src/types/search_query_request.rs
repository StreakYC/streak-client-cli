pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for Search
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchQueryRequest {
    /// Exact box name to search for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Zero-based page number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Pipeline keys to constrain the search to.
    #[serde(rename = "pipelineKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_key: Option<Vec<String>>,
    /// Full-text query to search across boxes, contacts, and organizations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Stage keys to constrain the search to.
    #[serde(rename = "stageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_key: Option<Vec<String>>,
    /// Team keys to constrain the search to.
    #[serde(rename = "teamKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_key: Option<Vec<String>>,
}

impl SearchQueryRequest {
    pub fn builder() -> SearchQueryRequestBuilder {
        <SearchQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchQueryRequestBuilder {
    name: Option<String>,
    page: Option<i64>,
    pipeline_key: Option<Vec<String>>,
    query: Option<String>,
    stage_key: Option<Vec<String>>,
    team_key: Option<Vec<String>>,
}

impl SearchQueryRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn pipeline_key(mut self, value: Vec<String>) -> Self {
        self.pipeline_key = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn stage_key(mut self, value: Vec<String>) -> Self {
        self.stage_key = Some(value);
        self
    }

    pub fn team_key(mut self, value: Vec<String>) -> Self {
        self.team_key = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchQueryRequest`].
    pub fn build(self) -> Result<SearchQueryRequest, BuildError> {
        Ok(SearchQueryRequest {
            name: self.name,
            page: self.page,
            pipeline_key: self.pipeline_key,
            query: self.query,
            stage_key: self.stage_key,
            team_key: self.team_key,
        })
    }
}

