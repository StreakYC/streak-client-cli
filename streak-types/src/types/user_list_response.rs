pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A wrapper for a list of User(s) that supports paging when available.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserListResponse {
    /// Whether there are more User(s) available after this response.
    #[serde(rename = "hasNextPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_next_page: Option<bool>,
    /// The User(s) returned by this response.
    #[serde(default)]
    pub results: Vec<User>,
}

impl UserListResponse {
    pub fn builder() -> UserListResponseBuilder {
        <UserListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserListResponseBuilder {
    has_next_page: Option<bool>,
    results: Option<Vec<User>>,
}

impl UserListResponseBuilder {
    pub fn has_next_page(mut self, value: bool) -> Self {
        self.has_next_page = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<User>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](UserListResponseBuilder::results)
    pub fn build(self) -> Result<UserListResponse, BuildError> {
        Ok(UserListResponse {
            has_next_page: self.has_next_page,
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
