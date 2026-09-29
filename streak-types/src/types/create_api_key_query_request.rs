pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for createApiKey
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateApiKeyQueryRequest {
    #[serde(rename = "teamKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_key: Option<String>,
}

impl CreateApiKeyQueryRequest {
    pub fn builder() -> CreateApiKeyQueryRequestBuilder {
        <CreateApiKeyQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApiKeyQueryRequestBuilder {
    team_key: Option<String>,
}

impl CreateApiKeyQueryRequestBuilder {
    pub fn team_key(mut self, value: impl Into<String>) -> Self {
        self.team_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateApiKeyQueryRequest`].
    pub fn build(self) -> Result<CreateApiKeyQueryRequest, BuildError> {
        Ok(CreateApiKeyQueryRequest {
            team_key: self.team_key,
        })
    }
}

