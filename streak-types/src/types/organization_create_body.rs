pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrganizationCreateBody {
    /// Organization name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Freeform notes about the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
    /// Website domains for the organization, with the primary domain first.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// Industry of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    /// Phone numbers for the organization.
    #[serde(rename = "phoneNumbers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<String>>,
    /// Postal addresses for the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
    /// Company size as text, which can include an employee-count range.
    #[serde(rename = "employeeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_count: Option<String>,
    /// URL of the organization's logo.
    #[serde(rename = "logoURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Twitter profile handle or URL.
    #[serde(rename = "twitterHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_handle: Option<String>,
    /// Facebook profile handle or URL.
    #[serde(rename = "facebookHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_handle: Option<String>,
    /// LinkedIn profile handle or URL.
    #[serde(rename = "linkedinHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkedin_handle: Option<String>,
    /// Instagram profile handle or URL.
    #[serde(rename = "instagramHandle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instagram_handle: Option<String>,
    /// Return an existing organization matching a supplied domain instead of creating another organization. The supplied values are not merged into the existing organization.
    #[serde(rename = "getIfExisting")]
    #[serde(skip)]
    pub get_if_existing: Option<bool>,
}

impl OrganizationCreateBody {
    pub fn builder() -> OrganizationCreateBodyBuilder {
        <OrganizationCreateBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationCreateBodyBuilder {
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
    get_if_existing: Option<bool>,
}

impl OrganizationCreateBodyBuilder {
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

    pub fn get_if_existing(mut self, value: bool) -> Self {
        self.get_if_existing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationCreateBody`].
    pub fn build(self) -> Result<OrganizationCreateBody, BuildError> {
        Ok(OrganizationCreateBody {
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
            get_if_existing: self.get_if_existing,
        })
    }
}

