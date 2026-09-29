pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A contact shared with the team.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Contact {
    /// Stable key for this contact.
    #[serde(default)]
    pub key: String,
    /// Key for the team that owns this contact.
    #[serde(rename = "teamKey")]
    #[serde(default)]
    pub team_key: String,
    /// Given name of the contact.
    #[serde(rename = "givenName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    /// Family name of the contact.
    #[serde(rename = "familyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    /// Notes about the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
    /// Job title of the contact.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Normalized email addresses with duplicates removed, in first-occurrence order.
    #[serde(rename = "emailAddresses")]
    #[serde(default)]
    pub email_addresses: Vec<String>,
    /// Phone numbers with duplicates and null entries removed, in first-occurrence order.
    #[serde(rename = "phoneNumbers")]
    #[serde(default)]
    pub phone_numbers: Vec<String>,
    /// Postal addresses with duplicates and null entries removed, in first-occurrence order.
    #[serde(default)]
    pub addresses: Vec<String>,
    /// Website domains associated with the contact.
    #[serde(default)]
    pub domains: Vec<String>,
    /// Stored normalized domains used to match organizations.
    #[serde(rename = "normalizedDomains")]
    #[serde(default)]
    pub normalized_domains: Vec<String>,
    /// Twitter profile handle.
    #[serde(rename = "twitterHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_handle: Option<String>,
    /// Facebook profile handle.
    #[serde(rename = "facebookHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_handle: Option<String>,
    /// LinkedIn profile handle.
    #[serde(rename = "linkedinHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_handle: Option<String>,
    /// Instagram profile handle.
    #[serde(rename = "instagramHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram_handle: Option<String>,
    /// URL of the contact profile image.
    #[serde(rename = "photoUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// Key for the user who last edited this record.
    #[serde(rename = "lastSavedUserKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_saved_user_key: Option<String>,
    /// Key for the user who created this record.
    #[serde(rename = "creatorKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_key: Option<String>,
    /// Time this record was first created, in epoch milliseconds.
    #[serde(rename = "creationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<i64>,
    /// Time this version of the record was created, in epoch milliseconds.
    #[serde(rename = "versionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_timestamp: Option<i64>,
    /// Time this record was last saved, in epoch milliseconds.
    #[serde(rename = "lastSavedTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_saved_timestamp: Option<i64>,
    /// Time enrichment was last applied or reviewed, in epoch milliseconds.
    #[serde(rename = "lastEnrichmentTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_enrichment_timestamp: Option<i64>,
    /// Contacts linked to this record, in their stored order.
    #[serde(rename = "contactLinks")]
    #[serde(default)]
    pub contact_links: Vec<ContactLink>,
    /// Organizations linked to this record, in their stored order.
    #[serde(rename = "orgLinks")]
    #[serde(default)]
    pub org_links: Vec<OrganizationLink>,
    /// Custom field values keyed by field identifier.
    #[serde(default)]
    pub fields: HashMap<String, String>,
}

impl Contact {
    pub fn builder() -> ContactBuilder {
        <ContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactBuilder {
    key: Option<String>,
    team_key: Option<String>,
    given_name: Option<String>,
    family_name: Option<String>,
    other: Option<String>,
    title: Option<String>,
    email_addresses: Option<Vec<String>>,
    phone_numbers: Option<Vec<String>>,
    addresses: Option<Vec<String>>,
    domains: Option<Vec<String>>,
    normalized_domains: Option<Vec<String>>,
    twitter_handle: Option<String>,
    facebook_handle: Option<String>,
    linkedin_handle: Option<String>,
    instagram_handle: Option<String>,
    photo_url: Option<String>,
    last_saved_user_key: Option<String>,
    creator_key: Option<String>,
    creation_date: Option<i64>,
    version_timestamp: Option<i64>,
    last_saved_timestamp: Option<i64>,
    last_enrichment_timestamp: Option<i64>,
    contact_links: Option<Vec<ContactLink>>,
    org_links: Option<Vec<OrganizationLink>>,
    fields: Option<HashMap<String, String>>,
}

impl ContactBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn team_key(mut self, value: impl Into<String>) -> Self {
        self.team_key = Some(value.into());
        self
    }

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

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
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

    pub fn normalized_domains(mut self, value: Vec<String>) -> Self {
        self.normalized_domains = Some(value);
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

    pub fn last_saved_user_key(mut self, value: impl Into<String>) -> Self {
        self.last_saved_user_key = Some(value.into());
        self
    }

    pub fn creator_key(mut self, value: impl Into<String>) -> Self {
        self.creator_key = Some(value.into());
        self
    }

    pub fn creation_date(mut self, value: i64) -> Self {
        self.creation_date = Some(value);
        self
    }

    pub fn version_timestamp(mut self, value: i64) -> Self {
        self.version_timestamp = Some(value);
        self
    }

    pub fn last_saved_timestamp(mut self, value: i64) -> Self {
        self.last_saved_timestamp = Some(value);
        self
    }

    pub fn last_enrichment_timestamp(mut self, value: i64) -> Self {
        self.last_enrichment_timestamp = Some(value);
        self
    }

    pub fn contact_links(mut self, value: Vec<ContactLink>) -> Self {
        self.contact_links = Some(value);
        self
    }

    pub fn org_links(mut self, value: Vec<OrganizationLink>) -> Self {
        self.org_links = Some(value);
        self
    }

    pub fn fields(mut self, value: HashMap<String, String>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Contact`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](ContactBuilder::key)
    /// - [`team_key`](ContactBuilder::team_key)
    /// - [`email_addresses`](ContactBuilder::email_addresses)
    /// - [`phone_numbers`](ContactBuilder::phone_numbers)
    /// - [`addresses`](ContactBuilder::addresses)
    /// - [`domains`](ContactBuilder::domains)
    /// - [`normalized_domains`](ContactBuilder::normalized_domains)
    /// - [`contact_links`](ContactBuilder::contact_links)
    /// - [`org_links`](ContactBuilder::org_links)
    /// - [`fields`](ContactBuilder::fields)
    pub fn build(self) -> Result<Contact, BuildError> {
        Ok(Contact {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            team_key: self.team_key.ok_or_else(|| BuildError::missing_field("team_key"))?,
            given_name: self.given_name,
            family_name: self.family_name,
            other: self.other,
            title: self.title,
            email_addresses: self.email_addresses.ok_or_else(|| BuildError::missing_field("email_addresses"))?,
            phone_numbers: self.phone_numbers.ok_or_else(|| BuildError::missing_field("phone_numbers"))?,
            addresses: self.addresses.ok_or_else(|| BuildError::missing_field("addresses"))?,
            domains: self.domains.ok_or_else(|| BuildError::missing_field("domains"))?,
            normalized_domains: self.normalized_domains.ok_or_else(|| BuildError::missing_field("normalized_domains"))?,
            twitter_handle: self.twitter_handle,
            facebook_handle: self.facebook_handle,
            linkedin_handle: self.linkedin_handle,
            instagram_handle: self.instagram_handle,
            photo_url: self.photo_url,
            last_saved_user_key: self.last_saved_user_key,
            creator_key: self.creator_key,
            creation_date: self.creation_date,
            version_timestamp: self.version_timestamp,
            last_saved_timestamp: self.last_saved_timestamp,
            last_enrichment_timestamp: self.last_enrichment_timestamp,
            contact_links: self.contact_links.ok_or_else(|| BuildError::missing_field("contact_links"))?,
            org_links: self.org_links.ok_or_else(|| BuildError::missing_field("org_links"))?,
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}
