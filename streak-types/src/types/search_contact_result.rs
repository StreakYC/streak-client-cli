pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Contact result returned by the v1 search endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchContactResult {
    /// Contact given name.
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// Contact family name.
    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    /// Contact display name or other indexed text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
    /// Key for the matching contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Contact title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Contact street addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    /// Contact email addresses.
    #[serde(rename = "emailAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<String>>,
    /// Contact phone numbers.
    #[serde(rename = "phoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<String>>,
}

impl SearchContactResult {
    pub fn builder() -> SearchContactResultBuilder {
        <SearchContactResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchContactResultBuilder {
    given_name: Option<String>,
    family_name: Option<String>,
    other: Option<String>,
    key: Option<String>,
    title: Option<String>,
    addresses: Option<Vec<String>>,
    email_addresses: Option<Vec<String>>,
    phone_numbers: Option<Vec<String>>,
}

impl SearchContactResultBuilder {
    pub fn given_name(mut self, value: impl Into<String>) -> Self {
        self.given_name = Some(value.into());
        self
    }

    pub fn family_name(mut self, value: impl Into<String>) -> Self {
        self.family_name = Some(value.into());
        self
    }

    pub fn other(mut self, value: impl Into<String>) -> Self {
        self.other = Some(value.into());
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn addresses(mut self, value: Vec<String>) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn email_addresses(mut self, value: Vec<String>) -> Self {
        self.email_addresses = Some(value);
        self
    }

    pub fn phone_numbers(mut self, value: Vec<String>) -> Self {
        self.phone_numbers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchContactResult`].
    pub fn build(self) -> Result<SearchContactResult, BuildError> {
        Ok(SearchContactResult {
            given_name: self.given_name,
            family_name: self.family_name,
            other: self.other,
            key: self.key,
            title: self.title,
            addresses: self.addresses,
            email_addresses: self.email_addresses,
            phone_numbers: self.phone_numbers,
        })
    }
}
