pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommentReactionSummary {
    /// Emoji used for this reaction group.
    #[serde(default)]
    pub emoji: String,
    /// Number of users who reacted with this emoji.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Users who reacted with this emoji, ordered by oldest reaction first.
    #[serde(default)]
    pub users: Vec<UserBrief>,
}

impl CommentReactionSummary {
    pub fn builder() -> CommentReactionSummaryBuilder {
        <CommentReactionSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommentReactionSummaryBuilder {
    emoji: Option<String>,
    count: Option<i64>,
    users: Option<Vec<UserBrief>>,
}

impl CommentReactionSummaryBuilder {
    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn users(mut self, value: Vec<UserBrief>) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommentReactionSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`emoji`](CommentReactionSummaryBuilder::emoji)
    /// - [`users`](CommentReactionSummaryBuilder::users)
    pub fn build(self) -> Result<CommentReactionSummary, BuildError> {
        Ok(CommentReactionSummary {
            emoji: self.emoji.ok_or_else(|| BuildError::missing_field("emoji"))?,
            count: self.count,
            users: self.users.ok_or_else(|| BuildError::missing_field("users"))?,
        })
    }
}
