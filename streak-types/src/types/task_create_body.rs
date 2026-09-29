pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskCreateBody {
    /// Task text.
    #[serde(default)]
    pub text: String,
    /// Task due date, in epoch milliseconds. Use -1 to create a task with no due date.
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<i64>,
    /// Users to assign to this task. Omit to assign the task to the creator.
    #[serde(rename = "assignedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<Vec<AssigneeInput>>,
    /// Key for the meeting this task belongs to, when any.
    #[serde(rename = "meetingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_key: Option<String>,
    /// Whether to create this task as a draft. Draft tasks do not send notifications or webhooks.
    #[serde(rename = "isDraft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
    /// Key for the contact used to infer suggested task actions.
    #[serde(rename = "teamContactKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_contact_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
}

impl TaskCreateBody {
    pub fn builder() -> TaskCreateBodyBuilder {
        <TaskCreateBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskCreateBodyBuilder {
    text: Option<String>,
    due_date: Option<i64>,
    assigned_to: Option<Vec<AssigneeInput>>,
    meeting_key: Option<String>,
    is_draft: Option<bool>,
    team_contact_key: Option<String>,
    draft: Option<bool>,
}

impl TaskCreateBodyBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: i64) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn assigned_to(mut self, value: Vec<AssigneeInput>) -> Self {
        self.assigned_to = Some(value);
        self
    }

    pub fn meeting_key(mut self, value: impl Into<String>) -> Self {
        self.meeting_key = Some(value.into());
        self
    }

    pub fn is_draft(mut self, value: bool) -> Self {
        self.is_draft = Some(value);
        self
    }

    pub fn team_contact_key(mut self, value: impl Into<String>) -> Self {
        self.team_contact_key = Some(value.into());
        self
    }

    pub fn draft(mut self, value: bool) -> Self {
        self.draft = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskCreateBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](TaskCreateBodyBuilder::text)
    pub fn build(self) -> Result<TaskCreateBody, BuildError> {
        Ok(TaskCreateBody {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            due_date: self.due_date,
            assigned_to: self.assigned_to,
            meeting_key: self.meeting_key,
            is_draft: self.is_draft,
            team_contact_key: self.team_contact_key,
            draft: self.draft,
        })
    }
}

