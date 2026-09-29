pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionType {
    Email,
    Schedule,
    Call,
    FollowUp,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ActionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Email => serializer.serialize_str("EMAIL"),
            Self::Schedule => serializer.serialize_str("SCHEDULE"),
            Self::Call => serializer.serialize_str("CALL"),
            Self::FollowUp => serializer.serialize_str("FOLLOW_UP"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "EMAIL" => Ok(Self::Email),
            "SCHEDULE" => Ok(Self::Schedule),
            "CALL" => Ok(Self::Call),
            "FOLLOW_UP" => Ok(Self::FollowUp),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Email => write!(f, "EMAIL"),
            Self::Schedule => write!(f, "SCHEDULE"),
            Self::Call => write!(f, "CALL"),
            Self::FollowUp => write!(f, "FOLLOW_UP"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
