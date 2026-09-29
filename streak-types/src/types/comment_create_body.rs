pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommentCreateBody {
    /// Comment text.
    #[serde(default)]
    pub message: String,
    /// Mention ranges in the comment text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<CommentMentionBody>>,
    /// Parent comment key when creating a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Whether to scan the comment text for unstructured @mentions of users display name, first name or email address.
    #[serde(rename = "searchForAtMentions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_for_at_mentions: Option<bool>,
}

impl CommentCreateBody {
    pub fn builder() -> CommentCreateBodyBuilder {
        <CommentCreateBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommentCreateBodyBuilder {
    message: Option<String>,
    mentions: Option<Vec<CommentMentionBody>>,
    parent: Option<String>,
    search_for_at_mentions: Option<bool>,
}

impl CommentCreateBodyBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn mentions(mut self, value: Vec<CommentMentionBody>) -> Self {
        self.mentions = Some(value);
        self
    }

    pub fn parent(mut self, value: impl Into<String>) -> Self {
        self.parent = Some(value.into());
        self
    }

    pub fn search_for_at_mentions(mut self, value: bool) -> Self {
        self.search_for_at_mentions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommentCreateBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](CommentCreateBodyBuilder::message)
    pub fn build(self) -> Result<CommentCreateBody, BuildError> {
        Ok(CommentCreateBody {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            mentions: self.mentions,
            parent: self.parent,
            search_for_at_mentions: self.search_for_at_mentions,
        })
    }
}

