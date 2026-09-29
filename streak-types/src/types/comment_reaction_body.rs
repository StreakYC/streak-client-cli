pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommentReactionBody {
    /// Emoji to add or remove as a reaction.
    #[serde(default)]
    pub emoji: String,
}

impl CommentReactionBody {
    pub fn builder() -> CommentReactionBodyBuilder {
        <CommentReactionBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommentReactionBodyBuilder {
    emoji: Option<String>,
}

impl CommentReactionBodyBuilder {
    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CommentReactionBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`emoji`](CommentReactionBodyBuilder::emoji)
    pub fn build(self) -> Result<CommentReactionBody, BuildError> {
        Ok(CommentReactionBody {
            emoji: self.emoji.ok_or_else(|| BuildError::missing_field("emoji"))?,
        })
    }
}
