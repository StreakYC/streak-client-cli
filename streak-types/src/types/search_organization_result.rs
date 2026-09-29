pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Organization result returned by the v1 search endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchOrganizationResult {
    /// Organization name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Organization display name or other indexed text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,
    /// Key for the matching organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Organization industry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    /// Organization employee count label.
    #[serde(rename = "employeeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_count: Option<String>,
    /// Organization logo URL.
    #[serde(rename = "logoURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Normalized organization domains.
    #[serde(rename = "normalizedDomains")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized_domains: Option<Vec<String>>,
    /// Organization domains.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domains: Option<Vec<String>>,
    /// Organization street addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<String>>,
}

impl SearchOrganizationResult {
    pub fn builder() -> SearchOrganizationResultBuilder {
        <SearchOrganizationResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchOrganizationResultBuilder {
    name: Option<String>,
    other: Option<String>,
    key: Option<String>,
    industry: Option<String>,
    employee_count: Option<String>,
    logo_url: Option<String>,
    normalized_domains: Option<Vec<String>>,
    domains: Option<Vec<String>>,
    addresses: Option<Vec<String>>,
}

impl SearchOrganizationResultBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn industry(mut self, value: impl Into<String>) -> Self {
        self.industry = Some(value.into());
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

    pub fn normalized_domains(mut self, value: Vec<String>) -> Self {
        self.normalized_domains = Some(value);
        self
    }

    pub fn domains(mut self, value: Vec<String>) -> Self {
        self.domains = Some(value);
        self
    }

    pub fn addresses(mut self, value: Vec<String>) -> Self {
        self.addresses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchOrganizationResult`].
    pub fn build(self) -> Result<SearchOrganizationResult, BuildError> {
        Ok(SearchOrganizationResult {
            name: self.name,
            other: self.other,
            key: self.key,
            industry: self.industry,
            employee_count: self.employee_count,
            logo_url: self.logo_url,
            normalized_domains: self.normalized_domains,
            domains: self.domains,
            addresses: self.addresses,
        })
    }
}
