pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A Streak team and its membership, sharing, billing, and directory settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Team {
    /// Key for this team.
    #[serde(default)]
    pub key: String,
    /// Timestamp when the team was created, in epoch milliseconds.
    #[serde(rename = "creationDate")]
    #[serde(default)]
    pub creation_date: i64,
    /// Key for the user who created the team.
    #[serde(default)]
    pub creator: String,
    /// Members of the team and their team roles.
    #[serde(default)]
    pub members: Vec<TeamMember>,
    /// Display name for the team.
    #[serde(default)]
    pub name: String,
    /// Whether sharing outside the team is restricted.
    #[serde(rename = "sharingRestrictedToTeam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharing_restricted_to_team: Option<bool>,
    /// Whether requests to join the team are approved automatically.
    #[serde(rename = "automaticallyApproveJoinRequests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_approve_join_requests: Option<bool>,
    /// Whether invoice emails are sent automatically for this team.
    #[serde(rename = "automaticallySendInvoiceEmails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_send_invoice_emails: Option<bool>,
    /// Additional email addresses that receive team invoices.
    #[serde(rename = "emailsToSendInvoiceTo")]
    #[serde(default)]
    pub emails_to_send_invoice_to: Vec<String>,
    /// Key for the user who administers billing for this team.
    #[serde(rename = "billingAdmin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_admin: Option<String>,
    /// Timestamp when the team was last persisted, in epoch milliseconds.
    #[serde(rename = "lastSavedTimestamp")]
    #[serde(default)]
    pub last_saved_timestamp: i64,
    /// Permissions for the team's system contact and organization lists.
    #[serde(rename = "contactOrgListPermissions")]
    #[serde(default)]
    pub contact_org_list_permissions: HashMap<String, TeamBasedPermissions>,
    /// Custom contact field definitions for this team.
    #[serde(rename = "contactSettings")]
    #[serde(default)]
    pub contact_settings: TeamFieldSettings,
    /// Custom organization field definitions for this team.
    #[serde(rename = "organizationSettings")]
    #[serde(default)]
    pub organization_settings: TeamFieldSettings,
}

impl Team {
    pub fn builder() -> TeamBuilder {
        <TeamBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamBuilder {
    key: Option<String>,
    creation_date: Option<i64>,
    creator: Option<String>,
    members: Option<Vec<TeamMember>>,
    name: Option<String>,
    sharing_restricted_to_team: Option<bool>,
    automatically_approve_join_requests: Option<bool>,
    automatically_send_invoice_emails: Option<bool>,
    emails_to_send_invoice_to: Option<Vec<String>>,
    billing_admin: Option<String>,
    last_saved_timestamp: Option<i64>,
    contact_org_list_permissions: Option<HashMap<String, TeamBasedPermissions>>,
    contact_settings: Option<TeamFieldSettings>,
    organization_settings: Option<TeamFieldSettings>,
}

impl TeamBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn creation_date(mut self, value: i64) -> Self {
        self.creation_date = Some(value);
        self
    }

    pub fn creator(mut self, value: impl Into<String>) -> Self {
        self.creator = Some(value.into());
        self
    }

    pub fn members(mut self, value: Vec<TeamMember>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn sharing_restricted_to_team(mut self, value: bool) -> Self {
        self.sharing_restricted_to_team = Some(value);
        self
    }

    pub fn automatically_approve_join_requests(mut self, value: bool) -> Self {
        self.automatically_approve_join_requests = Some(value);
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

    pub fn billing_admin(mut self, value: impl Into<String>) -> Self {
        self.billing_admin = Some(value.into());
        self
    }

    pub fn last_saved_timestamp(mut self, value: i64) -> Self {
        self.last_saved_timestamp = Some(value);
        self
    }

    pub fn contact_org_list_permissions(mut self, value: HashMap<String, TeamBasedPermissions>) -> Self {
        self.contact_org_list_permissions = Some(value);
        self
    }

    pub fn contact_settings(mut self, value: TeamFieldSettings) -> Self {
        self.contact_settings = Some(value);
        self
    }

    pub fn organization_settings(mut self, value: TeamFieldSettings) -> Self {
        self.organization_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Team`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](TeamBuilder::key)
    /// - [`creation_date`](TeamBuilder::creation_date)
    /// - [`creator`](TeamBuilder::creator)
    /// - [`members`](TeamBuilder::members)
    /// - [`name`](TeamBuilder::name)
    /// - [`emails_to_send_invoice_to`](TeamBuilder::emails_to_send_invoice_to)
    /// - [`last_saved_timestamp`](TeamBuilder::last_saved_timestamp)
    /// - [`contact_org_list_permissions`](TeamBuilder::contact_org_list_permissions)
    /// - [`contact_settings`](TeamBuilder::contact_settings)
    /// - [`organization_settings`](TeamBuilder::organization_settings)
    pub fn build(self) -> Result<Team, BuildError> {
        Ok(Team {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            creation_date: self.creation_date.ok_or_else(|| BuildError::missing_field("creation_date"))?,
            creator: self.creator.ok_or_else(|| BuildError::missing_field("creator"))?,
            members: self.members.ok_or_else(|| BuildError::missing_field("members"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            sharing_restricted_to_team: self.sharing_restricted_to_team,
            automatically_approve_join_requests: self.automatically_approve_join_requests,
            automatically_send_invoice_emails: self.automatically_send_invoice_emails,
            emails_to_send_invoice_to: self.emails_to_send_invoice_to.ok_or_else(|| BuildError::missing_field("emails_to_send_invoice_to"))?,
            billing_admin: self.billing_admin,
            last_saved_timestamp: self.last_saved_timestamp.ok_or_else(|| BuildError::missing_field("last_saved_timestamp"))?,
            contact_org_list_permissions: self.contact_org_list_permissions.ok_or_else(|| BuildError::missing_field("contact_org_list_permissions"))?,
            contact_settings: self.contact_settings.ok_or_else(|| BuildError::missing_field("contact_settings"))?,
            organization_settings: self.organization_settings.ok_or_else(|| BuildError::missing_field("organization_settings"))?,
        })
    }
}
