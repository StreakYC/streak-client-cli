pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommentMentionBody {
    /// Key for the mentioned user.
    #[serde(rename = "userKey")]
    #[serde(default)]
    pub user_key: String,
    /// Mention start offset in the comment text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Mention length in the comment text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
}

impl CommentMentionBody {
    pub fn builder() -> CommentMentionBodyBuilder {
        <CommentMentionBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommentMentionBodyBuilder {
    user_key: Option<String>,
    offset: Option<i64>,
    length: Option<i64>,
}

impl CommentMentionBodyBuilder {
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

    /// Consumes the builder and constructs a [`CommentMentionBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`user_key`](CommentMentionBodyBuilder::user_key)
    pub fn build(self) -> Result<CommentMentionBody, BuildError> {
        Ok(CommentMentionBody {
            user_key: self.user_key.ok_or_else(|| BuildError::missing_field("user_key"))?,
            offset: self.offset,
            length: self.length,
        })
    }
}
