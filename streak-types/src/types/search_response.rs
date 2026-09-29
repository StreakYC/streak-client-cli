pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Response returned by the legacy v1 search endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchResponse {
    /// Search results grouped by entity type.
    #[serde(default)]
    pub results: SearchResults,
    /// The query string used for full-text searches. Null when searching by exact box name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Zero-based page number returned by the search.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl SearchResponse {
    pub fn builder() -> SearchResponseBuilder {
        <SearchResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchResponseBuilder {
    results: Option<SearchResults>,
    query: Option<String>,
    page: Option<i64>,
}

impl SearchResponseBuilder {
    pub fn results(mut self, value: SearchResults) -> Self {
        self.results = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](SearchResponseBuilder::results)
    pub fn build(self) -> Result<SearchResponse, BuildError> {
        Ok(SearchResponse {
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
            query: self.query,
            page: self.page,
        })
    }
}
