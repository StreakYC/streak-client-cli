pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A wrapper for a list of Team(s) that supports paging when available.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TeamListResponse {
    /// Whether there are more Team(s) available after this response.
    #[serde(rename = "hasNextPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_next_page: Option<bool>,
    /// The Team(s) returned by this response.
    #[serde(default)]
    pub results: Vec<Team>,
}

impl TeamListResponse {
    pub fn builder() -> TeamListResponseBuilder {
        <TeamListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamListResponseBuilder {
    has_next_page: Option<bool>,
    results: Option<Vec<Team>>,
}

impl TeamListResponseBuilder {
    pub fn has_next_page(mut self, value: bool) -> Self {
        self.has_next_page = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<Team>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](TeamListResponseBuilder::results)
    pub fn build(self) -> Result<TeamListResponse, BuildError> {
        Ok(TeamListResponse {
            has_next_page: self.has_next_page,
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
