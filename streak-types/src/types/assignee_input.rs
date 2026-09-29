pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Assignee identified by user key or email address. At least one must be provided. If both are specified, userKey takes precedence.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssigneeInput {
    /// Key for the assigned user. Takes precedence when both userKey and email are specified.
    #[serde(rename = "userKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_key: Option<String>,
    /// Email address for the assigned user. Required when userKey is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl AssigneeInput {
    pub fn builder() -> AssigneeInputBuilder {
        <AssigneeInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssigneeInputBuilder {
    user_key: Option<String>,
    email: Option<String>,
}

impl AssigneeInputBuilder {
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssigneeInput`].
    pub fn build(self) -> Result<AssigneeInput, BuildError> {
        Ok(AssigneeInput {
            user_key: self.user_key,
            email: self.email,
        })
    }
}
