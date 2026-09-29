pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApiKey {
    /// The API key secret value.
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
    /// Key for this API key.
    #[serde(default)]
    pub key: String,
    /// Timestamp when this API key was created, in epoch milliseconds.
    #[serde(rename = "creationTimestamp")]
    #[serde(default)]
    pub creation_timestamp: i64,
    /// Timestamp when the last time this key was modified, in epoch milliseconds.
    #[serde(rename = "lastSavedTimestamp")]
    #[serde(default)]
    pub last_saved_timestamp: i64,
    /// Whether this key authenticates one user or can impersonate members of a team.
    #[serde(rename = "keyType")]
    pub key_type: ApiKeyType,
    /// Team this key can impersonate members of. Only present for team keys.
    #[serde(rename = "teamKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_key: Option<String>,
}

impl ApiKey {
    pub fn builder() -> ApiKeyBuilder {
        <ApiKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyBuilder {
    api_key: Option<String>,
    key: Option<String>,
    creation_timestamp: Option<i64>,
    last_saved_timestamp: Option<i64>,
    key_type: Option<ApiKeyType>,
    team_key: Option<String>,
}

impl ApiKeyBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn creation_timestamp(mut self, value: i64) -> Self {
        self.creation_timestamp = Some(value);
        self
    }

    pub fn last_saved_timestamp(mut self, value: i64) -> Self {
        self.last_saved_timestamp = Some(value);
        self
    }

    pub fn key_type(mut self, value: ApiKeyType) -> Self {
        self.key_type = Some(value);
        self
    }

    pub fn team_key(mut self, value: impl Into<String>) -> Self {
        self.team_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApiKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_key`](ApiKeyBuilder::api_key)
    /// - [`key`](ApiKeyBuilder::key)
    /// - [`creation_timestamp`](ApiKeyBuilder::creation_timestamp)
    /// - [`last_saved_timestamp`](ApiKeyBuilder::last_saved_timestamp)
    /// - [`key_type`](ApiKeyBuilder::key_type)
    pub fn build(self) -> Result<ApiKey, BuildError> {
        Ok(ApiKey {
            api_key: self.api_key.ok_or_else(|| BuildError::missing_field("api_key"))?,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            creation_timestamp: self.creation_timestamp.ok_or_else(|| BuildError::missing_field("creation_timestamp"))?,
            last_saved_timestamp: self.last_saved_timestamp.ok_or_else(|| BuildError::missing_field("last_saved_timestamp"))?,
            key_type: self.key_type.ok_or_else(|| BuildError::missing_field("key_type"))?,
            team_key: self.team_key,
        })
    }
}
