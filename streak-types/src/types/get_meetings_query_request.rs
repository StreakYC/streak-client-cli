pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for getMeetings
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetMeetingsQueryRequest {
    /// Number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Zero-based page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl GetMeetingsQueryRequest {
    pub fn builder() -> GetMeetingsQueryRequestBuilder {
        <GetMeetingsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetMeetingsQueryRequestBuilder {
    limit: Option<i64>,
    page: Option<i64>,
}

impl GetMeetingsQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetMeetingsQueryRequest`].
    pub fn build(self) -> Result<GetMeetingsQueryRequest, BuildError> {
        Ok(GetMeetingsQueryRequest {
            limit: self.limit,
            page: self.page,
        })
    }
}

