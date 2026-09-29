pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A wrapper for a list of Pipeline(s) that supports paging when available.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PipelineListResponse {
    /// Whether there are more Pipeline(s) available after this response.
    #[serde(rename = "hasNextPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_next_page: Option<bool>,
    /// The Pipeline(s) returned by this response.
    #[serde(default)]
    pub results: Vec<Pipeline>,
}

impl PipelineListResponse {
    pub fn builder() -> PipelineListResponseBuilder {
        <PipelineListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PipelineListResponseBuilder {
    has_next_page: Option<bool>,
    results: Option<Vec<Pipeline>>,
}

impl PipelineListResponseBuilder {
    pub fn has_next_page(mut self, value: bool) -> Self {
        self.has_next_page = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<Pipeline>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PipelineListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](PipelineListResponseBuilder::results)
    pub fn build(self) -> Result<PipelineListResponse, BuildError> {
        Ok(PipelineListResponse {
            has_next_page: self.has_next_page,
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
