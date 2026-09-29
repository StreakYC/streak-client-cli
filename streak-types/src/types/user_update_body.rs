pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserUpdateBody {
    /// IANA timezone identifier for the user, such as America/Los_Angeles.
    #[serde(rename = "timezoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_id: Option<String>,
    /// Given name for the user.
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Family name for the user.
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Whether invoice emails should be sent automatically.
    #[serde(rename = "automaticallySendInvoiceEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_send_invoice_emails: Option<bool>,
    /// Who should receive invoices
    #[serde(rename = "emailsToSendInvoiceTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails_to_send_invoice_to: Option<Vec<String>>,
    /// Custom data that should be shown on the invoice
    #[serde(rename = "customerSpecifiedInvoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_specified_invoice_data: Option<String>,
}

impl UserUpdateBody {
    pub fn builder() -> UserUpdateBodyBuilder {
        <UserUpdateBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserUpdateBodyBuilder {
    timezone_id: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    automatically_send_invoice_emails: Option<bool>,
    emails_to_send_invoice_to: Option<Vec<String>>,
    customer_specified_invoice_data: Option<String>,
}

impl UserUpdateBodyBuilder {
    pub fn timezone_id(mut self, value: impl Into<String>) -> Self {
        self.timezone_id = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn automatically_send_invoice_emails(mut self, value: bool) -> Self {
        self.automatically_send_invoice_emails = Some(value);
        self
    }

    pub fn emails_to_send_invoice_to(mut self, value: Vec<String>) -> Self {
        self.emails_to_send_invoice_to = Some(value);
        self
    }

    pub fn customer_specified_invoice_data(mut self, value: impl Into<String>) -> Self {
        self.customer_specified_invoice_data = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserUpdateBody`].
    pub fn build(self) -> Result<UserUpdateBody, BuildError> {
        Ok(UserUpdateBody {
            timezone_id: self.timezone_id,
            first_name: self.first_name,
            last_name: self.last_name,
            automatically_send_invoice_emails: self.automatically_send_invoice_emails,
            emails_to_send_invoice_to: self.emails_to_send_invoice_to,
            customer_specified_invoice_data: self.customer_specified_invoice_data,
        })
    }
}

