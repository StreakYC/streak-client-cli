pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MeetingUpdateBody {
    /// New meeting type. Omit this field to keep the existing meeting type unchanged.
    #[serde(rename = "meetingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_type: Option<String>,
    /// New meeting start timestamp, in epoch milliseconds. Omit this field to keep the existing start time unchanged.
    #[serde(rename = "startTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<i64>,
    /// New meeting duration in milliseconds. Omit this field to keep the existing duration unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// New meeting notes. Omit this field to keep the existing notes unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Whether to publish a draft meeting. Omit this field to keep the existing draft state unchanged.
    #[serde(rename = "isDraft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
    /// Replacement task keys associated with this meeting. Omit this field to keep existing task keys unchanged.
    #[serde(rename = "taskKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_keys: Option<Vec<String>>,
}

impl MeetingUpdateBody {
    pub fn builder() -> MeetingUpdateBodyBuilder {
        <MeetingUpdateBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MeetingUpdateBodyBuilder {
    meeting_type: Option<String>,
    start_timestamp: Option<i64>,
    duration: Option<i64>,
    notes: Option<String>,
    is_draft: Option<bool>,
    task_keys: Option<Vec<String>>,
}

impl MeetingUpdateBodyBuilder {
    pub fn meeting_type(mut self, value: impl Into<String>) -> Self {
        self.meeting_type = Some(value.into());
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

    pub fn is_draft(mut self, value: bool) -> Self {
        self.is_draft = Some(value);
        self
    }

    pub fn task_keys(mut self, value: Vec<String>) -> Self {
        self.task_keys = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MeetingUpdateBody`].
    pub fn build(self) -> Result<MeetingUpdateBody, BuildError> {
        Ok(MeetingUpdateBody {
            meeting_type: self.meeting_type,
            start_timestamp: self.start_timestamp,
            duration: self.duration,
            notes: self.notes,
            is_draft: self.is_draft,
            task_keys: self.task_keys,
        })
    }
}

