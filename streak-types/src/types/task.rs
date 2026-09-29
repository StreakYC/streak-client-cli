pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Task attached to a box, including assignment, due date, status, and suggested actions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Task {
    /// Key for this task.
    #[serde(default)]
    pub key: String,
    /// Key for the box this task belongs to.
    #[serde(rename = "boxKey")]
    #[serde(default)]
    pub box_key: String,
    /// Key for the pipeline this task belongs to.
    #[serde(rename = "pipelineKey")]
    #[serde(default)]
    pub pipeline_key: String,
    /// Key for the user who created this task.
    #[serde(rename = "creatorKey")]
    #[serde(default)]
    pub creator_key: String,
    /// The user who created this task.
    #[serde(default)]
    pub creator: UserBrief,
    /// Users assigned to this task. Tasks created without explicit assignees are assigned to the creator by default.
    #[serde(rename = "assignedTo")]
    #[serde(default)]
    pub assigned_to: Vec<UserBrief>,
    /// Timestamp when this task was created, in epoch milliseconds.
    #[serde(rename = "creationDate")]
    #[serde(default)]
    pub creation_date: i64,
    /// Timestamp when this task was last persisted, in epoch milliseconds.
    #[serde(rename = "lastSavedTimestamp")]
    #[serde(default)]
    pub last_saved_timestamp: i64,
    /// Timestamp when this task status last changed, in epoch milliseconds.
    #[serde(rename = "lastStatusChangeDate")]
    #[serde(default)]
    pub last_status_change_date: i64,
    /// Task due date, in epoch milliseconds.
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<i64>,
    /// Task text.
    #[serde(default)]
    pub text: String,
    /// Suggested actions inferred from the task text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<TaskAction>>,
    /// Task completion status.
    pub status: TaskStatus,
    /// Task reminder status.
    #[serde(rename = "reminderStatus")]
    pub reminder_status: ReminderStatus,
    /// Key for the meeting this task belongs to, when any.
    #[serde(rename = "meetingKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_key: Option<String>,
}

impl Task {
    pub fn builder() -> TaskBuilder {
        <TaskBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskBuilder {
    key: Option<String>,
    box_key: Option<String>,
    pipeline_key: Option<String>,
    creator_key: Option<String>,
    creator: Option<UserBrief>,
    assigned_to: Option<Vec<UserBrief>>,
    creation_date: Option<i64>,
    last_saved_timestamp: Option<i64>,
    last_status_change_date: Option<i64>,
    due_date: Option<i64>,
    text: Option<String>,
    actions: Option<Vec<TaskAction>>,
    status: Option<TaskStatus>,
    reminder_status: Option<ReminderStatus>,
    meeting_key: Option<String>,
}

impl TaskBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn box_key(mut self, value: impl Into<String>) -> Self {
        self.box_key = Some(value.into());
        self
    }

    pub fn pipeline_key(mut self, value: impl Into<String>) -> Self {
        self.pipeline_key = Some(value.into());
        self
    }

    pub fn creator_key(mut self, value: impl Into<String>) -> Self {
        self.creator_key = Some(value.into());
        self
    }

    pub fn creator(mut self, value: UserBrief) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn assigned_to(mut self, value: Vec<UserBrief>) -> Self {
        self.assigned_to = Some(value);
        self
    }

    pub fn creation_date(mut self, value: i64) -> Self {
        self.creation_date = Some(value);
        self
    }

    pub fn last_saved_timestamp(mut self, value: i64) -> Self {
        self.last_saved_timestamp = Some(value);
        self
    }

    pub fn last_status_change_date(mut self, value: i64) -> Self {
        self.last_status_change_date = Some(value);
        self
    }

    pub fn due_date(mut self, value: i64) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn actions(mut self, value: Vec<TaskAction>) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn status(mut self, value: TaskStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn reminder_status(mut self, value: ReminderStatus) -> Self {
        self.reminder_status = Some(value);
        self
    }

    pub fn meeting_key(mut self, value: impl Into<String>) -> Self {
        self.meeting_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Task`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](TaskBuilder::key)
    /// - [`box_key`](TaskBuilder::box_key)
    /// - [`pipeline_key`](TaskBuilder::pipeline_key)
    /// - [`creator_key`](TaskBuilder::creator_key)
    /// - [`creator`](TaskBuilder::creator)
    /// - [`assigned_to`](TaskBuilder::assigned_to)
    /// - [`creation_date`](TaskBuilder::creation_date)
    /// - [`last_saved_timestamp`](TaskBuilder::last_saved_timestamp)
    /// - [`last_status_change_date`](TaskBuilder::last_status_change_date)
    /// - [`text`](TaskBuilder::text)
    /// - [`status`](TaskBuilder::status)
    /// - [`reminder_status`](TaskBuilder::reminder_status)
    pub fn build(self) -> Result<Task, BuildError> {
        Ok(Task {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            box_key: self.box_key.ok_or_else(|| BuildError::missing_field("box_key"))?,
            pipeline_key: self.pipeline_key.ok_or_else(|| BuildError::missing_field("pipeline_key"))?,
            creator_key: self.creator_key.ok_or_else(|| BuildError::missing_field("creator_key"))?,
            creator: self.creator.ok_or_else(|| BuildError::missing_field("creator"))?,
            assigned_to: self.assigned_to.ok_or_else(|| BuildError::missing_field("assigned_to"))?,
            creation_date: self.creation_date.ok_or_else(|| BuildError::missing_field("creation_date"))?,
            last_saved_timestamp: self.last_saved_timestamp.ok_or_else(|| BuildError::missing_field("last_saved_timestamp"))?,
            last_status_change_date: self.last_status_change_date.ok_or_else(|| BuildError::missing_field("last_status_change_date"))?,
            due_date: self.due_date,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            actions: self.actions,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            reminder_status: self.reminder_status.ok_or_else(|| BuildError::missing_field("reminder_status"))?,
            meeting_key: self.meeting_key,
        })
    }
}
