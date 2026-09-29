pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A page of contact records and its continuation cursor.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ContactPage {
    /// Records in this page, in result order.
    #[serde(default)]
    pub results: Vec<Contact>,
    /// Opaque cursor to pass to the next page request, when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ContactPage {
    pub fn builder() -> ContactPageBuilder {
        <ContactPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactPageBuilder {
    results: Option<Vec<Contact>>,
    cursor: Option<String>,
}

impl ContactPageBuilder {
    pub fn results(mut self, value: Vec<Contact>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ContactPage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](ContactPageBuilder::results)
    pub fn build(self) -> Result<ContactPage, BuildError> {
        Ok(ContactPage {
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
            cursor: self.cursor,
        })
    }
}
