pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for listContacts
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListContactsQueryRequest {
    /// Include contacts last saved at or after this epoch-seconds timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<i64>,
    /// Cursor returned by the previous response. Omit or leave blank to start from the beginning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum number of results to return. Defaults to 100. Values above 1000 are capped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListContactsQueryRequest {
    pub fn builder() -> ListContactsQueryRequestBuilder {
        <ListContactsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListContactsQueryRequestBuilder {
    after: Option<i64>,
    cursor: Option<String>,
    limit: Option<i64>,
}

impl ListContactsQueryRequestBuilder {
    pub fn after(mut self, value: i64) -> Self {
        self.after = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListContactsQueryRequest`].
    pub fn build(self) -> Result<ListContactsQueryRequest, BuildError> {
        Ok(ListContactsQueryRequest {
            after: self.after,
            cursor: self.cursor,
            limit: self.limit,
        })
    }
}

