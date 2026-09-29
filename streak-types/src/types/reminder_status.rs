pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReminderStatus {
    None,
    Scheduled,
    Reminded,
    ErrorOnReminder,
    NotRemindedBecauseTaskDone,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReminderStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("NONE"),
            Self::Scheduled => serializer.serialize_str("SCHEDULED"),
            Self::Reminded => serializer.serialize_str("REMINDED"),
            Self::ErrorOnReminder => serializer.serialize_str("ERROR_ON_REMINDER"),
            Self::NotRemindedBecauseTaskDone => serializer.serialize_str("NOT_REMINDED_BECAUSE_TASK_DONE"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReminderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "NONE" => Ok(Self::None),
            "SCHEDULED" => Ok(Self::Scheduled),
            "REMINDED" => Ok(Self::Reminded),
            "ERROR_ON_REMINDER" => Ok(Self::ErrorOnReminder),
            "NOT_REMINDED_BECAUSE_TASK_DONE" => Ok(Self::NotRemindedBecauseTaskDone),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReminderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::Scheduled => write!(f, "SCHEDULED"),
            Self::Reminded => write!(f, "REMINDED"),
            Self::ErrorOnReminder => write!(f, "ERROR_ON_REMINDER"),
            Self::NotRemindedBecauseTaskDone => write!(f, "NOT_REMINDED_BECAUSE_TASK_DONE"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
