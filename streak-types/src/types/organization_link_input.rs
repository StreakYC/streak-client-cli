pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A relationship to an organization.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationLinkInput {
    /// Key of the related organization.
    #[serde(default)]
    pub key: String,
    /// Whether the relationship was created automatically.
    #[serde(rename = "isMagicLinked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_magic_linked: Option<bool>,
    /// Relationship creation timestamp, in epoch milliseconds, when known.
    #[serde(rename = "creationTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_timestamp: Option<i64>,
}

impl OrganizationLinkInput {
    pub fn builder() -> OrganizationLinkInputBuilder {
        <OrganizationLinkInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationLinkInputBuilder {
    key: Option<String>,
    is_magic_linked: Option<bool>,
    creation_timestamp: Option<i64>,
}

impl OrganizationLinkInputBuilder {
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

    /// Consumes the builder and constructs a [`OrganizationLinkInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](OrganizationLinkInputBuilder::key)
    pub fn build(self) -> Result<OrganizationLinkInput, BuildError> {
        Ok(OrganizationLinkInput {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            is_magic_linked: self.is_magic_linked,
            creation_timestamp: self.creation_timestamp,
        })
    }
}
