pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for getBoxMarkdown
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetBoxMarkdownQueryRequest {
    /// A token cap for the entire document, counted with OpenAI's o200k_base encoding. It covers the box sections first, then uses the remaining tokens for the timeline. A marker shows where entries were left out. If the box does not fit under the cap, a 400 error shows the minimum it needs. Omit the cap to return the full box.
    #[serde(rename = "maxTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
}

impl GetBoxMarkdownQueryRequest {
    pub fn builder() -> GetBoxMarkdownQueryRequestBuilder {
        <GetBoxMarkdownQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBoxMarkdownQueryRequestBuilder {
    max_tokens: Option<i64>,
}

impl GetBoxMarkdownQueryRequestBuilder {
    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBoxMarkdownQueryRequest`].
    pub fn build(self) -> Result<GetBoxMarkdownQueryRequest, BuildError> {
        Ok(GetBoxMarkdownQueryRequest {
            max_tokens: self.max_tokens,
        })
    }
}

