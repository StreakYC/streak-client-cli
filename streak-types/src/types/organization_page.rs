pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A page of organization records and its continuation cursor.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OrganizationPage {
    /// Records in this page, in result order.
    #[serde(default)]
    pub results: Vec<Organization>,
    /// Opaque cursor to pass to the next page request, when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl OrganizationPage {
    pub fn builder() -> OrganizationPageBuilder {
        <OrganizationPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationPageBuilder {
    results: Option<Vec<Organization>>,
    cursor: Option<String>,
}

impl OrganizationPageBuilder {
    pub fn results(mut self, value: Vec<Organization>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrganizationPage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](OrganizationPageBuilder::results)
    pub fn build(self) -> Result<OrganizationPage, BuildError> {
        Ok(OrganizationPage {
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
            cursor: self.cursor,
        })
    }
}
