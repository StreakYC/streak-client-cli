pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A relationship to an organization.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationLink {
    /// Stable key for the linked organization.
    #[serde(default)]
    pub key: String,
    /// Whether Streak linked this record automatically.
    #[serde(rename = "isMagicLinked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_magic_linked: Option<bool>,
    /// Time the relationship was created, in epoch milliseconds.
    #[serde(rename = "creationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<i64>,
}

impl OrganizationLink {
    pub fn builder() -> OrganizationLinkBuilder {
        <OrganizationLinkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationLinkBuilder {
    key: Option<String>,
    is_magic_linked: Option<bool>,
    creation_timestamp: Option<i64>,
}

impl OrganizationLinkBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn is_magic_linked(mut self, value: bool) -> Self {
        self.is_magic_linked = Some(value);
        self
    }

    pub fn creation_timestamp(mut self, value: i64) -> Self {
        self.creation_timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationLink`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](OrganizationLinkBuilder::key)
    pub fn build(self) -> Result<OrganizationLink, BuildError> {
        Ok(OrganizationLink {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            is_magic_linked: self.is_magic_linked,
            creation_timestamp: self.creation_timestamp,
        })
    }
}
