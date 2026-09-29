pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Comment {
    /// Key for this comment.
    #[serde(default)]
    pub key: String,
    /// Key for the box this comment belongs to.
    #[serde(rename = "boxKey")]
    #[serde(default)]
    pub box_key: String,
    /// Key for the pipeline this comment belongs to.
    #[serde(rename = "pipelineKey")]
    #[serde(default)]
    pub pipeline_key: String,
    /// Key for the user who created this comment.
    #[serde(rename = "creatorKey")]
    #[serde(default)]
    pub creator_key: String,
    /// Key for the newsfeed entry created for this comment.
    #[serde(rename = "newsfeedEntryKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub newsfeed_entry_key: Option<String>,
    /// Timestamp when the comment was created, in epoch milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    /// Comment text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// User mentions found in the comment text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<CommentMention>>,
    /// Key for the parent comment, when this comment is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Keys for this comment's direct replies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
    /// Emoji reactions grouped by emoji.
    #[serde(rename = "reactionSummary")]
    #[serde(default)]
    pub reaction_summary: Vec<CommentReactionSummary>,
    /// Timestamp when the comment was last persisted, in epoch milliseconds.
    #[serde(rename = "lastSavedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_saved_timestamp: Option<i64>,
}

impl Comment {
    pub fn builder() -> CommentBuilder {
        <CommentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommentBuilder {
    key: Option<String>,
    box_key: Option<String>,
    pipeline_key: Option<String>,
    creator_key: Option<String>,
    newsfeed_entry_key: Option<String>,
    timestamp: Option<i64>,
    message: Option<String>,
    mentions: Option<Vec<CommentMention>>,
    parent: Option<String>,
    children: Option<Vec<String>>,
    reaction_summary: Option<Vec<CommentReactionSummary>>,
    last_saved_timestamp: Option<i64>,
}

impl CommentBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn box_key(mut self, value: impl Into<String>) -> Self {
        self.box_key = Some(value.into());
        self
    }

    pub fn pipeline_key(mut self, value: impl Into<String>) -> Self {
        self.pipeline_key = Some(value.into());
        self
    }

    pub fn creator_key(mut self, value: impl Into<String>) -> Self {
        self.creator_key = Some(value.into());
        self
    }

    pub fn newsfeed_entry_key(mut self, value: impl Into<String>) -> Self {
        self.newsfeed_entry_key = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: i64) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn mentions(mut self, value: Vec<CommentMention>) -> Self {
        self.mentions = Some(value);
        self
    }

    pub fn parent(mut self, value: impl Into<String>) -> Self {
        self.parent = Some(value.into());
        self
    }

    pub fn children(mut self, value: Vec<String>) -> Self {
        self.children = Some(value);
        self
    }

    pub fn reaction_summary(mut self, value: Vec<CommentReactionSummary>) -> Self {
        self.reaction_summary = Some(value);
        self
    }

    pub fn last_saved_timestamp(mut self, value: i64) -> Self {
        self.last_saved_timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Comment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](CommentBuilder::key)
    /// - [`box_key`](CommentBuilder::box_key)
    /// - [`pipeline_key`](CommentBuilder::pipeline_key)
    /// - [`creator_key`](CommentBuilder::creator_key)
    /// - [`reaction_summary`](CommentBuilder::reaction_summary)
    pub fn build(self) -> Result<Comment, BuildError> {
        Ok(Comment {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            box_key: self.box_key.ok_or_else(|| BuildError::missing_field("box_key"))?,
            pipeline_key: self.pipeline_key.ok_or_else(|| BuildError::missing_field("pipeline_key"))?,
            creator_key: self.creator_key.ok_or_else(|| BuildError::missing_field("creator_key"))?,
            newsfeed_entry_key: self.newsfeed_entry_key,
            timestamp: self.timestamp,
            message: self.message,
            mentions: self.mentions,
            parent: self.parent,
            children: self.children,
            reaction_summary: self.reaction_summary.ok_or_else(|| BuildError::missing_field("reaction_summary"))?,
            last_saved_timestamp: self.last_saved_timestamp,
        })
    }
}
