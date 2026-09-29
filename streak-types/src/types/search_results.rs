pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Search results grouped by legacy result collection name.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchResults {
    /// Matching boxes.
    #[serde(default)]
    pub boxes: Vec<SearchBoxResult>,
    /// Matching contacts.
    #[serde(default)]
    pub contacts: Vec<SearchContactResult>,
    /// Matching organizations.
    #[serde(default)]
    pub orgs: Vec<SearchOrganizationResult>,
}

impl SearchResults {
    pub fn builder() -> SearchResultsBuilder {
        <SearchResultsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchResultsBuilder {
    boxes: Option<Vec<SearchBoxResult>>,
    contacts: Option<Vec<SearchContactResult>>,
    orgs: Option<Vec<SearchOrganizationResult>>,
}

impl SearchResultsBuilder {
    pub fn boxes(mut self, value: Vec<SearchBoxResult>) -> Self {
        self.boxes = Some(value);
        self
    }

    pub fn contacts(mut self, value: Vec<SearchContactResult>) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn orgs(mut self, value: Vec<SearchOrganizationResult>) -> Self {
        self.orgs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchResults`].
    /// This method will fail if any of the following fields are not set:
    /// - [`boxes`](SearchResultsBuilder::boxes)
    /// - [`contacts`](SearchResultsBuilder::contacts)
    /// - [`orgs`](SearchResultsBuilder::orgs)
    pub fn build(self) -> Result<SearchResults, BuildError> {
        Ok(SearchResults {
            boxes: self.boxes.ok_or_else(|| BuildError::missing_field("boxes"))?,
            contacts: self.contacts.ok_or_else(|| BuildError::missing_field("contacts"))?,
            orgs: self.orgs.ok_or_else(|| BuildError::missing_field("orgs"))?,
        })
    }
}
