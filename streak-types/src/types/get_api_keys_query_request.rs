pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for getApiKeys
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetApiKeysQueryRequest {
    /// Number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Zero-based page number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

impl GetApiKeysQueryRequest {
    pub fn builder() -> GetApiKeysQueryRequestBuilder {
        <GetApiKeysQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetApiKeysQueryRequestBuilder {
    limit: Option<i64>,
    page: Option<i64>,
}

impl GetApiKeysQueryRequestBuilder {
    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetApiKeysQueryRequest`].
    pub fn build(self) -> Result<GetApiKeysQueryRequest, BuildError> {
        Ok(GetApiKeysQueryRequest {
            limit: self.limit,
            page: self.page,
        })
    }
}

