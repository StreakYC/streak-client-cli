pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Role {
    Member,
    Owner,
    PayerOnly,
    ArchiveOnly,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Role {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Member => serializer.serialize_str("Member"),
            Self::Owner => serializer.serialize_str("Owner"),
            Self::PayerOnly => serializer.serialize_str("PayerOnly"),
            Self::ArchiveOnly => serializer.serialize_str("ArchiveOnly"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Member" => Ok(Self::Member),
            "Owner" => Ok(Self::Owner),
            "PayerOnly" => Ok(Self::PayerOnly),
            "ArchiveOnly" => Ok(Self::ArchiveOnly),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Member => write!(f, "Member"),
            Self::Owner => write!(f, "Owner"),
            Self::PayerOnly => write!(f, "PayerOnly"),
            Self::ArchiveOnly => write!(f, "ArchiveOnly"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
