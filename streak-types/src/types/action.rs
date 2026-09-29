pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    Get,
    Update,
    Delete,
    Create,
    ChangePermissions,
    ChangeAdmins,
    SetTeam,
    RestrictSharing,
    OverrideSharingRestriction,
    OverrideValidator,
    Resize,
    Hide,
    Freeze,
    Options,
    Format,
    Order,
    Summary,
    Export,
    FlowPauseResume,
    FlowViewLogs,
    GetBoxes,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Action {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Get => serializer.serialize_str("GET"),
            Self::Update => serializer.serialize_str("UPDATE"),
            Self::Delete => serializer.serialize_str("DELETE"),
            Self::Create => serializer.serialize_str("CREATE"),
            Self::ChangePermissions => serializer.serialize_str("CHANGE_PERMISSIONS"),
            Self::ChangeAdmins => serializer.serialize_str("CHANGE_ADMINS"),
            Self::SetTeam => serializer.serialize_str("SET_TEAM"),
            Self::RestrictSharing => serializer.serialize_str("RESTRICT_SHARING"),
            Self::OverrideSharingRestriction => serializer.serialize_str("OVERRIDE_SHARING_RESTRICTION"),
            Self::OverrideValidator => serializer.serialize_str("OVERRIDE_VALIDATOR"),
            Self::Resize => serializer.serialize_str("RESIZE"),
            Self::Hide => serializer.serialize_str("HIDE"),
            Self::Freeze => serializer.serialize_str("FREEZE"),
            Self::Options => serializer.serialize_str("OPTIONS"),
            Self::Format => serializer.serialize_str("FORMAT"),
            Self::Order => serializer.serialize_str("ORDER"),
            Self::Summary => serializer.serialize_str("SUMMARY"),
            Self::Export => serializer.serialize_str("EXPORT"),
            Self::FlowPauseResume => serializer.serialize_str("FLOW_PAUSE_RESUME"),
            Self::FlowViewLogs => serializer.serialize_str("FLOW_VIEW_LOGS"),
            Self::GetBoxes => serializer.serialize_str("GET_BOXES"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Action {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "GET" => Ok(Self::Get),
            "UPDATE" => Ok(Self::Update),
            "DELETE" => Ok(Self::Delete),
            "CREATE" => Ok(Self::Create),
            "CHANGE_PERMISSIONS" => Ok(Self::ChangePermissions),
            "CHANGE_ADMINS" => Ok(Self::ChangeAdmins),
            "SET_TEAM" => Ok(Self::SetTeam),
            "RESTRICT_SHARING" => Ok(Self::RestrictSharing),
            "OVERRIDE_SHARING_RESTRICTION" => Ok(Self::OverrideSharingRestriction),
            "OVERRIDE_VALIDATOR" => Ok(Self::OverrideValidator),
            "RESIZE" => Ok(Self::Resize),
            "HIDE" => Ok(Self::Hide),
            "FREEZE" => Ok(Self::Freeze),
            "OPTIONS" => Ok(Self::Options),
            "FORMAT" => Ok(Self::Format),
            "ORDER" => Ok(Self::Order),
            "SUMMARY" => Ok(Self::Summary),
            "EXPORT" => Ok(Self::Export),
            "FLOW_PAUSE_RESUME" => Ok(Self::FlowPauseResume),
            "FLOW_VIEW_LOGS" => Ok(Self::FlowViewLogs),
            "GET_BOXES" => Ok(Self::GetBoxes),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Update => write!(f, "UPDATE"),
            Self::Delete => write!(f, "DELETE"),
            Self::Create => write!(f, "CREATE"),
            Self::ChangePermissions => write!(f, "CHANGE_PERMISSIONS"),
            Self::ChangeAdmins => write!(f, "CHANGE_ADMINS"),
            Self::SetTeam => write!(f, "SET_TEAM"),
            Self::RestrictSharing => write!(f, "RESTRICT_SHARING"),
            Self::OverrideSharingRestriction => write!(f, "OVERRIDE_SHARING_RESTRICTION"),
            Self::OverrideValidator => write!(f, "OVERRIDE_VALIDATOR"),
            Self::Resize => write!(f, "RESIZE"),
            Self::Hide => write!(f, "HIDE"),
            Self::Freeze => write!(f, "FREEZE"),
            Self::Options => write!(f, "OPTIONS"),
            Self::Format => write!(f, "FORMAT"),
            Self::Order => write!(f, "ORDER"),
            Self::Summary => write!(f, "SUMMARY"),
            Self::Export => write!(f, "EXPORT"),
            Self::FlowPauseResume => write!(f, "FLOW_PAUSE_RESUME"),
            Self::FlowViewLogs => write!(f, "FLOW_VIEW_LOGS"),
            Self::GetBoxes => write!(f, "GET_BOXES"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
