pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ContactUpdate {
    /// New given name; an empty string clears it if another identifier remains.
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// New family name; an empty string clears it if another identifier remains.
    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    /// New job title; an empty string clears it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// New notes; an empty string clears them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
    /// Replacement email addresses; a contact must retain an email address or a name.
    #[serde(rename = "emailAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_addresses: Option<Vec<String>>,
    /// Replacement phone numbers. Empty entries are removed.
    #[serde(rename = "phoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<String>>,
    /// Replacement postal addresses. Empty entries are removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    /// Replacement website domains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// Replacement Twitter handle.
    #[serde(rename = "twitterHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_handle: Option<String>,
    /// Replacement Facebook handle.
    #[serde(rename = "facebookHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_handle: Option<String>,
    /// Replacement LinkedIn handle.
    #[serde(rename = "linkedinHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_handle: Option<String>,
    /// Replacement Instagram handle.
    #[serde(rename = "instagramHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram_handle: Option<String>,
    /// Replacement photo URL.
    #[serde(rename = "photoUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Custom-field values to merge by field key. Null entries are ignored; an empty map leaves existing fields unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, Option<String>>>,
    /// Replacement links to contacts; an empty list removes the links.
    #[serde(rename = "contactLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_links: Option<Vec<ContactLinkInput>>,
    /// Replacement links to organizations; an empty list removes the links.
    #[serde(rename = "orgLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_links: Option<Vec<OrganizationLinkInput>>,
}

impl ContactUpdate {
    pub fn builder() -> ContactUpdateBuilder {
        <ContactUpdateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactUpdateBuilder {
    given_name: Option<String>,
    family_name: Option<String>,
    title: Option<String>,
    other: Option<String>,
    email_addresses: Option<Vec<String>>,
    phone_numbers: Option<Vec<String>>,
    addresses: Option<Vec<String>>,
    domains: Option<Vec<String>>,
    twitter_handle: Option<String>,
    facebook_handle: Option<String>,
    linkedin_handle: Option<String>,
    instagram_handle: Option<String>,
    photo_url: Option<String>,
    fields: Option<HashMap<String, Option<String>>>,
    contact_links: Option<Vec<ContactLinkInput>>,
    org_links: Option<Vec<OrganizationLinkInput>>,
}

impl ContactUpdateBuilder {
    pub fn given_name(mut self, value: impl Into<String>) -> Self {
        self.given_name = Some(value.into());
        self
    }

    pub fn family_name(mut self, value: impl Into<String>) -> Self {
        self.family_name = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn other(mut self, value: impl Into<String>) -> Self {
        self.other = Some(value.into());
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

    pub fn addresses(mut self, value: Vec<String>) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn domains(mut self, value: Vec<String>) -> Self {
        self.domains = Some(value);
        self
    }

    pub fn twitter_handle(mut self, value: impl Into<String>) -> Self {
        self.twitter_handle = Some(value.into());
        self
    }

    pub fn facebook_handle(mut self, value: impl Into<String>) -> Self {
        self.facebook_handle = Some(value.into());
        self
    }

    pub fn linkedin_handle(mut self, value: impl Into<String>) -> Self {
        self.linkedin_handle = Some(value.into());
        self
    }

    pub fn instagram_handle(mut self, value: impl Into<String>) -> Self {
        self.instagram_handle = Some(value.into());
        self
    }

    pub fn photo_url(mut self, value: impl Into<String>) -> Self {
        self.photo_url = Some(value.into());
        self
    }

    pub fn fields(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    pub fn contact_links(mut self, value: Vec<ContactLinkInput>) -> Self {
        self.contact_links = Some(value);
        self
    }

    pub fn org_links(mut self, value: Vec<OrganizationLinkInput>) -> Self {
        self.org_links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ContactUpdate`].
    pub fn build(self) -> Result<ContactUpdate, BuildError> {
        Ok(ContactUpdate {
            given_name: self.given_name,
            family_name: self.family_name,
            title: self.title,
            other: self.other,
            email_addresses: self.email_addresses,
            phone_numbers: self.phone_numbers,
            addresses: self.addresses,
            domains: self.domains,
            twitter_handle: self.twitter_handle,
            facebook_handle: self.facebook_handle,
            linkedin_handle: self.linkedin_handle,
            instagram_handle: self.instagram_handle,
            photo_url: self.photo_url,
            fields: self.fields,
            contact_links: self.contact_links,
            org_links: self.org_links,
        })
    }
}

