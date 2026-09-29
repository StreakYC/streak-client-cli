pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OrganizationUpdateBody {
    /// Replacement organization name. A name or domain must remain after the update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Replacement notes. An empty string clears the notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
    /// Replacement domains, with the primary domain first. An empty list clears domains if a name remains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// Replacement industry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    /// Replacement phone numbers. An empty list clears existing phone numbers.
    #[serde(rename = "phoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<String>>,
    /// Replacement postal addresses. An empty list clears existing addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    /// Replacement company size as text, including employee-count ranges.
    #[serde(rename = "employeeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_count: Option<String>,
    /// Replacement organization logo URL.
    #[serde(rename = "logoURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Replacement Twitter profile handle or URL.
    #[serde(rename = "twitterHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_handle: Option<String>,
    /// Replacement Facebook profile handle or URL.
    #[serde(rename = "facebookHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_handle: Option<String>,
    /// Replacement LinkedIn profile handle or URL.
    #[serde(rename = "linkedinHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_handle: Option<String>,
    /// Replacement Instagram profile handle or URL.
    #[serde(rename = "instagramHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram_handle: Option<String>,
    /// Replacement contact relationships. An empty list removes existing relationships.
    #[serde(rename = "contactLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_links: Option<Vec<ContactLinkInput>>,
    /// Replacement organization relationships. An empty list removes existing relationships.
    #[serde(rename = "orgLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_links: Option<Vec<OrganizationLinkInput>>,
    /// Custom field values to change, keyed by field ID. Omitted keys and null values are unchanged; an empty object makes no changes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<HashMap<String, Option<String>>>,
}

impl OrganizationUpdateBody {
    pub fn builder() -> OrganizationUpdateBodyBuilder {
        <OrganizationUpdateBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationUpdateBodyBuilder {
    name: Option<String>,
    other: Option<String>,
    domains: Option<Vec<String>>,
    industry: Option<String>,
    phone_numbers: Option<Vec<String>>,
    addresses: Option<Vec<String>>,
    employee_count: Option<String>,
    logo_url: Option<String>,
    twitter_handle: Option<String>,
    facebook_handle: Option<String>,
    linkedin_handle: Option<String>,
    instagram_handle: Option<String>,
    contact_links: Option<Vec<ContactLinkInput>>,
    org_links: Option<Vec<OrganizationLinkInput>>,
    fields: Option<HashMap<String, Option<String>>>,
}

impl OrganizationUpdateBodyBuilder {
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

    pub fn contact_links(mut self, value: Vec<ContactLinkInput>) -> Self {
        self.contact_links = Some(value);
        self
    }

    pub fn org_links(mut self, value: Vec<OrganizationLinkInput>) -> Self {
        self.org_links = Some(value);
        self
    }

    pub fn fields(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationUpdateBody`].
    pub fn build(self) -> Result<OrganizationUpdateBody, BuildError> {
        Ok(OrganizationUpdateBody {
            name: self.name,
            other: self.other,
            domains: self.domains,
            industry: self.industry,
            phone_numbers: self.phone_numbers,
            addresses: self.addresses,
            employee_count: self.employee_count,
            logo_url: self.logo_url,
            twitter_handle: self.twitter_handle,
            facebook_handle: self.facebook_handle,
            linkedin_handle: self.linkedin_handle,
            instagram_handle: self.instagram_handle,
            contact_links: self.contact_links,
            org_links: self.org_links,
            fields: self.fields,
        })
    }
}

