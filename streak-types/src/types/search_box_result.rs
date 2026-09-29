pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Box result returned by the v1 search endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchBoxResult {
    /// Key for the matching box.
    #[serde(rename = "boxKey")]
    #[serde(default)]
    pub box_key: String,
    /// Box name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Timestamp when the box was last updated, in epoch milliseconds.
    #[serde(rename = "lastUpdatedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_timestamp: Option<i64>,
    /// User keys assigned to the box.
    #[serde(rename = "assignedToKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to_keys: Option<Vec<String>>,
    /// Current stage key for the box.
    #[serde(rename = "stageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_key: Option<String>,
    /// Pipeline key for the box.
    #[serde(rename = "pipelineKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_key: Option<String>,
}

impl SearchBoxResult {
    pub fn builder() -> SearchBoxResultBuilder {
        <SearchBoxResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchBoxResultBuilder {
    box_key: Option<String>,
    name: Option<String>,
    last_updated_timestamp: Option<i64>,
    assigned_to_keys: Option<Vec<String>>,
    stage_key: Option<String>,
    pipeline_key: Option<String>,
}

impl SearchBoxResultBuilder {
    pub fn box_key(mut self, value: impl Into<String>) -> Self {
        self.box_key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn last_updated_timestamp(mut self, value: i64) -> Self {
        self.last_updated_timestamp = Some(value);
        self
    }

    pub fn assigned_to_keys(mut self, value: Vec<String>) -> Self {
        self.assigned_to_keys = Some(value);
        self
    }

    pub fn stage_key(mut self, value: impl Into<String>) -> Self {
        self.stage_key = Some(value.into());
        self
    }

    pub fn pipeline_key(mut self, value: impl Into<String>) -> Self {
        self.pipeline_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SearchBoxResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`box_key`](SearchBoxResultBuilder::box_key)
    pub fn build(self) -> Result<SearchBoxResult, BuildError> {
        Ok(SearchBoxResult {
            box_key: self.box_key.ok_or_else(|| BuildError::missing_field("box_key"))?,
            name: self.name,
            last_updated_timestamp: self.last_updated_timestamp,
            assigned_to_keys: self.assigned_to_keys,
            stage_key: self.stage_key,
            pipeline_key: self.pipeline_key,
        })
    }
}
