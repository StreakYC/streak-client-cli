pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Condition required for a permission to apply. NONE has no additional condition; ASSIGNED_TO and CREATOR match the user to the entity; BOX_UPDATE and BOX_VIEW require the corresponding permission on the containing pipeline item; UNARCHIVED requires an unarchived item. FIELD_COMPARISON and STAGE are reserved and currently unsupported.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipelineConditionalCheck {
    None,
    AssignedTo,
    FieldComparison,
    Stage,
    Creator,
    BoxUpdate,
    BoxView,
    Unarchived,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PipelineConditionalCheck {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::None => serializer.serialize_str("NONE"),
            Self::AssignedTo => serializer.serialize_str("ASSIGNED_TO"),
            Self::FieldComparison => serializer.serialize_str("FIELD_COMPARISON"),
            Self::Stage => serializer.serialize_str("STAGE"),
            Self::Creator => serializer.serialize_str("CREATOR"),
            Self::BoxUpdate => serializer.serialize_str("BOX_UPDATE"),
            Self::BoxView => serializer.serialize_str("BOX_VIEW"),
            Self::Unarchived => serializer.serialize_str("UNARCHIVED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PipelineConditionalCheck {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "NONE" => Ok(Self::None),
            "ASSIGNED_TO" => Ok(Self::AssignedTo),
            "FIELD_COMPARISON" => Ok(Self::FieldComparison),
            "STAGE" => Ok(Self::Stage),
            "CREATOR" => Ok(Self::Creator),
            "BOX_UPDATE" => Ok(Self::BoxUpdate),
            "BOX_VIEW" => Ok(Self::BoxView),
            "UNARCHIVED" => Ok(Self::Unarchived),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PipelineConditionalCheck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::AssignedTo => write!(f, "ASSIGNED_TO"),
            Self::FieldComparison => write!(f, "FIELD_COMPARISON"),
            Self::Stage => write!(f, "STAGE"),
            Self::Creator => write!(f, "CREATOR"),
            Self::BoxUpdate => write!(f, "BOX_UPDATE"),
            Self::BoxView => write!(f, "BOX_VIEW"),
            Self::Unarchived => write!(f, "UNARCHIVED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
