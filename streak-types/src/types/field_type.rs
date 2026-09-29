pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldType {
    TextInput,
    Dropdown,
    Checkbox,
    Date,
    Person,
    Formula,
    Tag,
    TeamContact,
    TeamOrganization,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FieldType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TextInput => serializer.serialize_str("TEXT_INPUT"),
            Self::Dropdown => serializer.serialize_str("DROPDOWN"),
            Self::Checkbox => serializer.serialize_str("CHECKBOX"),
            Self::Date => serializer.serialize_str("DATE"),
            Self::Person => serializer.serialize_str("PERSON"),
            Self::Formula => serializer.serialize_str("FORMULA"),
            Self::Tag => serializer.serialize_str("TAG"),
            Self::TeamContact => serializer.serialize_str("TEAM_CONTACT"),
            Self::TeamOrganization => serializer.serialize_str("TEAM_ORGANIZATION"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FieldType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "TEXT_INPUT" => Ok(Self::TextInput),
            "DROPDOWN" => Ok(Self::Dropdown),
            "CHECKBOX" => Ok(Self::Checkbox),
            "DATE" => Ok(Self::Date),
            "PERSON" => Ok(Self::Person),
            "FORMULA" => Ok(Self::Formula),
            "TAG" => Ok(Self::Tag),
            "TEAM_CONTACT" => Ok(Self::TeamContact),
            "TEAM_ORGANIZATION" => Ok(Self::TeamOrganization),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TextInput => write!(f, "TEXT_INPUT"),
            Self::Dropdown => write!(f, "DROPDOWN"),
            Self::Checkbox => write!(f, "CHECKBOX"),
            Self::Date => write!(f, "DATE"),
            Self::Person => write!(f, "PERSON"),
            Self::Formula => write!(f, "FORMULA"),
            Self::Tag => write!(f, "TAG"),
            Self::TeamContact => write!(f, "TEAM_CONTACT"),
            Self::TeamOrganization => write!(f, "TEAM_ORGANIZATION"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
