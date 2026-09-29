pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TaskUpdateBody {
    /// New task text. Omit this field to keep existing text unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// New task due date, in epoch milliseconds. Use -1 to clear the due date.
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<i64>,
    /// New task completion status. Omit this field to keep existing status unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Replacement users assigned to this task. Omit this field to keep existing assignees unchanged.
    #[serde(rename = "assignedTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<Vec<AssigneeInput>>,
    /// Key for the meeting this task belongs to. Omit this field to keep existing meeting unchanged.
    #[serde(rename = "meetingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_key: Option<String>,
    /// Whether to publish a draft task. Omit this field to keep existing draft state unchanged.
    #[serde(rename = "isDraft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
}

impl TaskUpdateBody {
    pub fn builder() -> TaskUpdateBodyBuilder {
        <TaskUpdateBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskUpdateBodyBuilder {
    text: Option<String>,
    due_date: Option<i64>,
    status: Option<String>,
    assigned_to: Option<Vec<AssigneeInput>>,
    meeting_key: Option<String>,
    is_draft: Option<bool>,
    draft: Option<bool>,
}

impl TaskUpdateBodyBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: i64) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
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

    pub fn draft(mut self, value: bool) -> Self {
        self.draft = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TaskUpdateBody`].
    pub fn build(self) -> Result<TaskUpdateBody, BuildError> {
        Ok(TaskUpdateBody {
            text: self.text,
            due_date: self.due_date,
            status: self.status,
            assigned_to: self.assigned_to,
            meeting_key: self.meeting_key,
            is_draft: self.is_draft,
            draft: self.draft,
        })
    }
}

