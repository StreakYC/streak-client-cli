pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommentMention {
    /// Key for the mentioned user.
    #[serde(rename = "userKey")]
    #[serde(default)]
    pub user_key: String,
    /// Zero-based character offset where the mention starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Character length of the mention.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
}

impl CommentMention {
    pub fn builder() -> CommentMentionBuilder {
        <CommentMentionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommentMentionBuilder {
    user_key: Option<String>,
    offset: Option<i64>,
    length: Option<i64>,
}

impl CommentMentionBuilder {
    pub fn user_key(mut self, value: impl Into<String>) -> Self {
        self.user_key = Some(value.into());
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    pub fn length(mut self, value: i64) -> Self {
        self.length = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommentMention`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_key`](CommentMentionBuilder::user_key)
    pub fn build(self) -> Result<CommentMention, BuildError> {
        Ok(CommentMention {
            user_key: self.user_key.ok_or_else(|| BuildError::missing_field("user_key"))?,
            offset: self.offset,
            length: self.length,
        })
    }
}
