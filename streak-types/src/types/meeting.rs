pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Meeting {
    /// Key for this meeting.
    #[serde(default)]
    pub key: String,
    /// Key for the box this meeting belongs to.
    #[serde(rename = "boxKey")]
    #[serde(default)]
    pub box_key: String,
    /// Key for the pipeline this meeting belongs to.
    #[serde(rename = "pipelineKey")]
    #[serde(default)]
    pub pipeline_key: String,
    /// Key for the user who created this meeting.
    #[serde(rename = "creatorKey")]
    #[serde(default)]
    pub creator_key: String,
    /// Whether this meeting is a draft.
    #[serde(rename = "isDraft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
    /// Task keys associated with this meeting.
    #[serde(rename = "taskKeys")]
    #[serde(default)]
    pub task_keys: Vec<String>,
    /// Timestamp when this meeting was created, in epoch milliseconds.
    #[serde(rename = "creationDate")]
    #[serde(default)]
    pub creation_date: i64,
    /// Meeting type.
    #[serde(rename = "meetingType")]
    pub meeting_type: MeetingType,
    /// Meeting start timestamp, in epoch milliseconds.
    #[serde(rename = "startTimestamp")]
    #[serde(default)]
    pub start_timestamp: i64,
    /// Meeting duration in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Meeting notes.
    #[serde(default)]
    pub notes: String,
    /// Timestamp when this meeting was last persisted, in epoch milliseconds.
    #[serde(rename = "lastSavedTimestamp")]
    #[serde(default)]
    pub last_saved_timestamp: i64,
}

impl Meeting {
    pub fn builder() -> MeetingBuilder {
        <MeetingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeetingBuilder {
    key: Option<String>,
    box_key: Option<String>,
    pipeline_key: Option<String>,
    creator_key: Option<String>,
    is_draft: Option<bool>,
    task_keys: Option<Vec<String>>,
    creation_date: Option<i64>,
    meeting_type: Option<MeetingType>,
    start_timestamp: Option<i64>,
    duration: Option<i64>,
    notes: Option<String>,
    last_saved_timestamp: Option<i64>,
}

impl MeetingBuilder {
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

    pub fn is_draft(mut self, value: bool) -> Self {
        self.is_draft = Some(value);
        self
    }

    pub fn task_keys(mut self, value: Vec<String>) -> Self {
        self.task_keys = Some(value);
        self
    }

    pub fn creation_date(mut self, value: i64) -> Self {
        self.creation_date = Some(value);
        self
    }

    pub fn meeting_type(mut self, value: MeetingType) -> Self {
        self.meeting_type = Some(value);
        self
    }

    pub fn start_timestamp(mut self, value: i64) -> Self {
        self.start_timestamp = Some(value);
        self
    }

    pub fn duration(mut self, value: i64) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn last_saved_timestamp(mut self, value: i64) -> Self {
        self.last_saved_timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Meeting`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](MeetingBuilder::key)
    /// - [`box_key`](MeetingBuilder::box_key)
    /// - [`pipeline_key`](MeetingBuilder::pipeline_key)
    /// - [`creator_key`](MeetingBuilder::creator_key)
    /// - [`task_keys`](MeetingBuilder::task_keys)
    /// - [`creation_date`](MeetingBuilder::creation_date)
    /// - [`meeting_type`](MeetingBuilder::meeting_type)
    /// - [`start_timestamp`](MeetingBuilder::start_timestamp)
    /// - [`notes`](MeetingBuilder::notes)
    /// - [`last_saved_timestamp`](MeetingBuilder::last_saved_timestamp)
    pub fn build(self) -> Result<Meeting, BuildError> {
        Ok(Meeting {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            box_key: self.box_key.ok_or_else(|| BuildError::missing_field("box_key"))?,
            pipeline_key: self.pipeline_key.ok_or_else(|| BuildError::missing_field("pipeline_key"))?,
            creator_key: self.creator_key.ok_or_else(|| BuildError::missing_field("creator_key"))?,
            is_draft: self.is_draft,
            task_keys: self.task_keys.ok_or_else(|| BuildError::missing_field("task_keys"))?,
            creation_date: self.creation_date.ok_or_else(|| BuildError::missing_field("creation_date"))?,
            meeting_type: self.meeting_type.ok_or_else(|| BuildError::missing_field("meeting_type"))?,
            start_timestamp: self.start_timestamp.ok_or_else(|| BuildError::missing_field("start_timestamp"))?,
            duration: self.duration,
            notes: self.notes.ok_or_else(|| BuildError::missing_field("notes"))?,
            last_saved_timestamp: self.last_saved_timestamp.ok_or_else(|| BuildError::missing_field("last_saved_timestamp"))?,
        })
    }
}
