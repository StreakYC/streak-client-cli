pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MeetingCreateBody {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Task keys associated with this meeting.
    #[serde(rename = "taskKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_keys: Option<Vec<String>>,
    /// Whether this meeting is a draft.
    #[serde(rename = "isDraft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
}

impl MeetingCreateBody {
    pub fn builder() -> MeetingCreateBodyBuilder {
        <MeetingCreateBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeetingCreateBodyBuilder {
    meeting_type: Option<MeetingType>,
    start_timestamp: Option<i64>,
    duration: Option<i64>,
    notes: Option<String>,
    task_keys: Option<Vec<String>>,
    is_draft: Option<bool>,
}

impl MeetingCreateBodyBuilder {
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

    pub fn task_keys(mut self, value: Vec<String>) -> Self {
        self.task_keys = Some(value);
        self
    }

    pub fn is_draft(mut self, value: bool) -> Self {
        self.is_draft = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeetingCreateBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`meeting_type`](MeetingCreateBodyBuilder::meeting_type)
    /// - [`start_timestamp`](MeetingCreateBodyBuilder::start_timestamp)
    pub fn build(self) -> Result<MeetingCreateBody, BuildError> {
        Ok(MeetingCreateBody {
            meeting_type: self.meeting_type.ok_or_else(|| BuildError::missing_field("meeting_type"))?,
            start_timestamp: self.start_timestamp.ok_or_else(|| BuildError::missing_field("start_timestamp"))?,
            duration: self.duration,
            notes: self.notes,
            task_keys: self.task_keys,
            is_draft: self.is_draft,
        })
    }
}

