pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An organization shared with the team.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Organization {
    /// Stable key for this organization.
    #[serde(default)]
    pub key: String,
    /// Key for the team that owns this organization.
    #[serde(rename = "teamKey")]
    #[serde(default)]
    pub team_key: String,
    /// Name of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Notes about the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
    /// Website domains in their stored order.
    #[serde(default)]
    pub domains: Vec<String>,
    /// Stored normalized website domains.
    #[serde(rename = "normalizedDomains")]
    #[serde(default)]
    pub normalized_domains: Vec<String>,
    /// Industry of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    /// Phone numbers in their stored order.
    #[serde(rename = "phoneNumbers")]
    #[serde(default)]
    pub phone_numbers: Vec<String>,
    /// Postal addresses in their stored order.
    #[serde(default)]
    pub addresses: Vec<String>,
    /// Employee count or range as text.
    #[serde(rename = "employeeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_count: Option<String>,
    /// URL of the organization logo.
    #[serde(rename = "logoURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
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

impl Organization {
    pub fn builder() -> OrganizationBuilder {
        <OrganizationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationBuilder {
    key: Option<String>,
    team_key: Option<String>,
    name: Option<String>,
    other: Option<String>,
    domains: Option<Vec<String>>,
    normalized_domains: Option<Vec<String>>,
    industry: Option<String>,
    phone_numbers: Option<Vec<String>>,
    addresses: Option<Vec<String>>,
    employee_count: Option<String>,
    logo_url: Option<String>,
    twitter_handle: Option<String>,
    facebook_handle: Option<String>,
    linkedin_handle: Option<String>,
    instagram_handle: Option<String>,
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

impl OrganizationBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn team_key(mut self, value: impl Into<String>) -> Self {
        self.team_key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn other(mut self, value: impl Into<String>) -> Self {
        self.other = Some(value.into());
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

    pub fn industry(mut self, value: impl Into<String>) -> Self {
        self.industry = Some(value.into());
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

    pub fn employee_count(mut self, value: impl Into<String>) -> Self {
        self.employee_count = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
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

    /// Consumes the builder and constructs a [`Organization`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](OrganizationBuilder::key)
    /// - [`team_key`](OrganizationBuilder::team_key)
    /// - [`domains`](OrganizationBuilder::domains)
    /// - [`normalized_domains`](OrganizationBuilder::normalized_domains)
    /// - [`phone_numbers`](OrganizationBuilder::phone_numbers)
    /// - [`addresses`](OrganizationBuilder::addresses)
    /// - [`contact_links`](OrganizationBuilder::contact_links)
    /// - [`org_links`](OrganizationBuilder::org_links)
    /// - [`fields`](OrganizationBuilder::fields)
    pub fn build(self) -> Result<Organization, BuildError> {
        Ok(Organization {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            team_key: self.team_key.ok_or_else(|| BuildError::missing_field("team_key"))?,
            name: self.name,
            other: self.other,
            domains: self.domains.ok_or_else(|| BuildError::missing_field("domains"))?,
            normalized_domains: self.normalized_domains.ok_or_else(|| BuildError::missing_field("normalized_domains"))?,
            industry: self.industry,
            phone_numbers: self.phone_numbers.ok_or_else(|| BuildError::missing_field("phone_numbers"))?,
            addresses: self.addresses.ok_or_else(|| BuildError::missing_field("addresses"))?,
            employee_count: self.employee_count,
            logo_url: self.logo_url,
            twitter_handle: self.twitter_handle,
            facebook_handle: self.facebook_handle,
            linkedin_handle: self.linkedin_handle,
            instagram_handle: self.instagram_handle,
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
