pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A relationship to a contact.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContactLinkInput {
    /// Key of the related contact.
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

impl ContactLinkInput {
    pub fn builder() -> ContactLinkInputBuilder {
        <ContactLinkInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactLinkInputBuilder {
    key: Option<String>,
    is_magic_linked: Option<bool>,
    creation_timestamp: Option<i64>,
}

impl ContactLinkInputBuilder {
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

    /// Consumes the builder and constructs a [`ContactLinkInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](ContactLinkInputBuilder::key)
    pub fn build(self) -> Result<ContactLinkInput, BuildError> {
        Ok(ContactLinkInput {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            is_magic_linked: self.is_magic_linked,
            creation_timestamp: self.creation_timestamp,
        })
    }
}
