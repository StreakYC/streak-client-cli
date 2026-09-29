pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A wrapper for a list of Meeting(s) that supports paging when available.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeetingListResponse {
    /// Whether there are more Meeting(s) available after this response.
    #[serde(rename = "hasNextPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_next_page: Option<bool>,
    /// The Meeting(s) returned by this response.
    #[serde(default)]
    pub results: Vec<Meeting>,
}

impl MeetingListResponse {
    pub fn builder() -> MeetingListResponseBuilder {
        <MeetingListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeetingListResponseBuilder {
    has_next_page: Option<bool>,
    results: Option<Vec<Meeting>>,
}

impl MeetingListResponseBuilder {
    pub fn has_next_page(mut self, value: bool) -> Self {
        self.has_next_page = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<Meeting>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeetingListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](MeetingListResponseBuilder::results)
    pub fn build(self) -> Result<MeetingListResponse, BuildError> {
        Ok(MeetingListResponse {
            has_next_page: self.has_next_page,
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
