pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MeetingType {
    CallLog,
    MeetingNotes,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MeetingType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CallLog => serializer.serialize_str("CALL_LOG"),
            Self::MeetingNotes => serializer.serialize_str("MEETING_NOTES"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MeetingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "CALL_LOG" => Ok(Self::CallLog),
            "MEETING_NOTES" => Ok(Self::MeetingNotes),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MeetingType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CallLog => write!(f, "CALL_LOG"),
            Self::MeetingNotes => write!(f, "MEETING_NOTES"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
