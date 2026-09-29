pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CommentUpdateBody {
    /// New comment text. Omit this field to keep the existing comment text unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// New mention ranges for the comment text. Omit this field to keep existing mentions unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentions: Option<Vec<CommentMentionBody>>,
}

impl CommentUpdateBody {
    pub fn builder() -> CommentUpdateBodyBuilder {
        <CommentUpdateBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CommentUpdateBodyBuilder {
    message: Option<String>,
    mentions: Option<Vec<CommentMentionBody>>,
}

impl CommentUpdateBodyBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn mentions(mut self, value: Vec<CommentMentionBody>) -> Self {
        self.mentions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CommentUpdateBody`].
    pub fn build(self) -> Result<CommentUpdateBody, BuildError> {
        Ok(CommentUpdateBody {
            message: self.message,
            mentions: self.mentions,
        })
    }
}

