pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TeamUpdateBody {
    /// New display name for the team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
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
    /// Complete replacement for additional invoice-recipient email addresses. Omit to leave unchanged; send an empty set to clear.
    #[serde(rename = "emailsToSendInvoiceTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails_to_send_invoice_to: Option<Vec<String>>,
    /// Complete replacement roster. Omit this property to leave membership unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TeamMemberUpdate>>,
    /// Permission updates keyed by system list. Omitted list entries remain unchanged.
    #[serde(rename = "contactOrgListPermissions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_org_list_permissions: Option<HashMap<String, Option<TeamBasedPermissionsUpdate>>>,
    /// Complete replacement for the team's custom contact fields. Omit to leave unchanged.
    #[serde(rename = "contactSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_settings: Option<TeamFieldSettingsUpdate>,
    /// Complete replacement for the team's custom organization fields. Omit to leave unchanged.
    #[serde(rename = "organizationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_settings: Option<TeamFieldSettingsUpdate>,
}

impl TeamUpdateBody {
    pub fn builder() -> TeamUpdateBodyBuilder {
        <TeamUpdateBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamUpdateBodyBuilder {
    name: Option<String>,
    sharing_restricted_to_team: Option<bool>,
    automatically_approve_join_requests: Option<bool>,
    automatically_send_invoice_emails: Option<bool>,
    emails_to_send_invoice_to: Option<Vec<String>>,
    members: Option<Vec<TeamMemberUpdate>>,
    contact_org_list_permissions: Option<HashMap<String, Option<TeamBasedPermissionsUpdate>>>,
    contact_settings: Option<TeamFieldSettingsUpdate>,
    organization_settings: Option<TeamFieldSettingsUpdate>,
}

impl TeamUpdateBodyBuilder {
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

    pub fn members(mut self, value: Vec<TeamMemberUpdate>) -> Self {
        self.members = Some(value);
        self
    }

    pub fn contact_org_list_permissions(mut self, value: HashMap<String, Option<TeamBasedPermissionsUpdate>>) -> Self {
        self.contact_org_list_permissions = Some(value);
        self
    }

    pub fn contact_settings(mut self, value: TeamFieldSettingsUpdate) -> Self {
        self.contact_settings = Some(value);
        self
    }

    pub fn organization_settings(mut self, value: TeamFieldSettingsUpdate) -> Self {
        self.organization_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamUpdateBody`].
    pub fn build(self) -> Result<TeamUpdateBody, BuildError> {
        Ok(TeamUpdateBody {
            name: self.name,
            sharing_restricted_to_team: self.sharing_restricted_to_team,
            automatically_approve_join_requests: self.automatically_approve_join_requests,
            automatically_send_invoice_emails: self.automatically_send_invoice_emails,
            emails_to_send_invoice_to: self.emails_to_send_invoice_to,
            members: self.members,
            contact_org_list_permissions: self.contact_org_list_permissions,
            contact_settings: self.contact_settings,
            organization_settings: self.organization_settings,
        })
    }
}

