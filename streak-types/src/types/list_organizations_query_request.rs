pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for listOrganizations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListOrganizationsQueryRequest {
    /// Cursor returned by the previous response. Omit or leave blank to start from the beginning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum number of results to return. Defaults to 100. Values above 1000 are capped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListOrganizationsQueryRequest {
    pub fn builder() -> ListOrganizationsQueryRequestBuilder {
        <ListOrganizationsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListOrganizationsQueryRequestBuilder {
    cursor: Option<String>,
    limit: Option<i64>,
}

impl ListOrganizationsQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListOrganizationsQueryRequest`].
    pub fn build(self) -> Result<ListOrganizationsQueryRequest, BuildError> {
        Ok(ListOrganizationsQueryRequest {
            cursor: self.cursor,
            limit: self.limit,
        })
    }
}

