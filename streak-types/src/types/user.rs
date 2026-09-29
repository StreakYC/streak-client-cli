pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct User {
    /// The user's normalized email address.
    #[serde(default)]
    pub email: String,
    /// Timestamp when the user account was created, in epoch milliseconds.
    #[serde(rename = "creationTimestamp")]
    #[serde(default)]
    pub creation_timestamp: i64,
    /// Timestamp when the user entity was last updated, in epoch milliseconds.
    #[serde(rename = "lastUpdatedTimestamp")]
    #[serde(default)]
    pub last_updated_timestamp: i64,
    /// Timestamp when the user entity was last persisted, in epoch milliseconds.
    #[serde(rename = "lastSavedTimestamp")]
    #[serde(default)]
    pub last_saved_timestamp: i64,
    /// Timestamp when Streak last saw activity for the user, in epoch milliseconds.
    #[serde(rename = "lastSeenTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_timestamp: Option<i64>,
    /// Stable Google profile identifier associated with the user's OAuth account.
    #[serde(rename = "googleProfileId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_profile_id: Option<String>,
    /// URL of the user's Google profile photo, when available.
    #[serde(rename = "googleProfilePhotoUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_profile_photo_url: Option<String>,
    /// URL of the user's Google profile, when available.
    #[serde(rename = "googleProfileLink")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_profile_link: Option<String>,
    /// Full display name from the user's Google profile.
    #[serde(rename = "googleProfileFullName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_profile_full_name: Option<String>,
    /// Given name from the user's Google profile, when available.
    #[serde(rename = "googleProfileFirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_profile_first_name: Option<String>,
    /// Family name from the user's Google profile, when available.
    #[serde(rename = "googleProfileLastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_profile_last_name: Option<String>,
    /// IANA timezone identifier for the user, such as America/Los_Angeles.
    #[serde(rename = "timezoneId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_id: Option<String>,
    /// Type of Streak user account, such as a human user or an agent user.
    #[serde(rename = "userType")]
    pub user_type: UserType,
    /// Whether security report emails are enabled for the user's team pipelines.
    #[serde(rename = "securityReportOnTeamsPipelines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_report_on_teams_pipelines: Option<bool>,
    /// Whether external sharing is restricted on the user's team pipelines.
    #[serde(rename = "externalSharingRestrictionOnTeamsPipelines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_sharing_restriction_on_teams_pipelines: Option<bool>,
    /// Whether invoice emails should be sent automatically for the user's billing context.
    #[serde(rename = "automaticallySendInvoiceEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_send_invoice_emails: Option<bool>,
    /// Additional email addresses that should receive invoice emails.
    #[serde(rename = "emailsToSendInvoiceTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails_to_send_invoice_to: Option<Vec<String>>,
    /// Customer-provided invoice data to include with billing records.
    #[serde(rename = "customerSpecifiedInvoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_specified_invoice_data: Option<String>,
    /// Timestamp when the user first completed OAuth, in epoch milliseconds.
    #[serde(rename = "firstOauthTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_oauth_timestamp: Option<i64>,
    /// Whether the user wants to receive task digest emails.
    #[serde(rename = "wantsTaskDigestEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wants_task_digest_email: Option<bool>,
    /// Display name Streak uses for the user in UI surfaces.
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    /// Key for this user.
    #[serde(default)]
    pub key: String,
}

impl User {
    pub fn builder() -> UserBuilder {
        <UserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBuilder {
    email: Option<String>,
    creation_timestamp: Option<i64>,
    last_updated_timestamp: Option<i64>,
    last_saved_timestamp: Option<i64>,
    last_seen_timestamp: Option<i64>,
    google_profile_id: Option<String>,
    google_profile_photo_url: Option<String>,
    google_profile_link: Option<String>,
    google_profile_full_name: Option<String>,
    google_profile_first_name: Option<String>,
    google_profile_last_name: Option<String>,
    timezone_id: Option<String>,
    user_type: Option<UserType>,
    security_report_on_teams_pipelines: Option<bool>,
    external_sharing_restriction_on_teams_pipelines: Option<bool>,
    automatically_send_invoice_emails: Option<bool>,
    emails_to_send_invoice_to: Option<Vec<String>>,
    customer_specified_invoice_data: Option<String>,
    first_oauth_timestamp: Option<i64>,
    wants_task_digest_email: Option<bool>,
    display_name: Option<String>,
    key: Option<String>,
}

impl UserBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn creation_timestamp(mut self, value: i64) -> Self {
        self.creation_timestamp = Some(value);
        self
    }

    pub fn last_updated_timestamp(mut self, value: i64) -> Self {
        self.last_updated_timestamp = Some(value);
        self
    }

    pub fn last_saved_timestamp(mut self, value: i64) -> Self {
        self.last_saved_timestamp = Some(value);
        self
    }

    pub fn last_seen_timestamp(mut self, value: i64) -> Self {
        self.last_seen_timestamp = Some(value);
        self
    }

    pub fn google_profile_id(mut self, value: impl Into<String>) -> Self {
        self.google_profile_id = Some(value.into());
        self
    }

    pub fn google_profile_photo_url(mut self, value: impl Into<String>) -> Self {
        self.google_profile_photo_url = Some(value.into());
        self
    }

    pub fn google_profile_link(mut self, value: impl Into<String>) -> Self {
        self.google_profile_link = Some(value.into());
        self
    }

    pub fn google_profile_full_name(mut self, value: impl Into<String>) -> Self {
        self.google_profile_full_name = Some(value.into());
        self
    }

    pub fn google_profile_first_name(mut self, value: impl Into<String>) -> Self {
        self.google_profile_first_name = Some(value.into());
        self
    }

    pub fn google_profile_last_name(mut self, value: impl Into<String>) -> Self {
        self.google_profile_last_name = Some(value.into());
        self
    }

    pub fn timezone_id(mut self, value: impl Into<String>) -> Self {
        self.timezone_id = Some(value.into());
        self
    }

    pub fn user_type(mut self, value: UserType) -> Self {
        self.user_type = Some(value);
        self
    }

    pub fn security_report_on_teams_pipelines(mut self, value: bool) -> Self {
        self.security_report_on_teams_pipelines = Some(value);
        self
    }

    pub fn external_sharing_restriction_on_teams_pipelines(mut self, value: bool) -> Self {
        self.external_sharing_restriction_on_teams_pipelines = Some(value);
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

    pub fn first_oauth_timestamp(mut self, value: i64) -> Self {
        self.first_oauth_timestamp = Some(value);
        self
    }

    pub fn wants_task_digest_email(mut self, value: bool) -> Self {
        self.wants_task_digest_email = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`User`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](UserBuilder::email)
    /// - [`creation_timestamp`](UserBuilder::creation_timestamp)
    /// - [`last_updated_timestamp`](UserBuilder::last_updated_timestamp)
    /// - [`last_saved_timestamp`](UserBuilder::last_saved_timestamp)
    /// - [`user_type`](UserBuilder::user_type)
    /// - [`display_name`](UserBuilder::display_name)
    /// - [`key`](UserBuilder::key)
    pub fn build(self) -> Result<User, BuildError> {
        Ok(User {
            email: self.email.ok_or_else(|| BuildError::missing_field("email"))?,
            creation_timestamp: self.creation_timestamp.ok_or_else(|| BuildError::missing_field("creation_timestamp"))?,
            last_updated_timestamp: self.last_updated_timestamp.ok_or_else(|| BuildError::missing_field("last_updated_timestamp"))?,
            last_saved_timestamp: self.last_saved_timestamp.ok_or_else(|| BuildError::missing_field("last_saved_timestamp"))?,
            last_seen_timestamp: self.last_seen_timestamp,
            google_profile_id: self.google_profile_id,
            google_profile_photo_url: self.google_profile_photo_url,
            google_profile_link: self.google_profile_link,
            google_profile_full_name: self.google_profile_full_name,
            google_profile_first_name: self.google_profile_first_name,
            google_profile_last_name: self.google_profile_last_name,
            timezone_id: self.timezone_id,
            user_type: self.user_type.ok_or_else(|| BuildError::missing_field("user_type"))?,
            security_report_on_teams_pipelines: self.security_report_on_teams_pipelines,
            external_sharing_restriction_on_teams_pipelines: self.external_sharing_restriction_on_teams_pipelines,
            automatically_send_invoice_emails: self.automatically_send_invoice_emails,
            emails_to_send_invoice_to: self.emails_to_send_invoice_to,
            customer_specified_invoice_data: self.customer_specified_invoice_data,
            first_oauth_timestamp: self.first_oauth_timestamp,
            wants_task_digest_email: self.wants_task_digest_email,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
        })
    }
}
