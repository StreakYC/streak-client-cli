pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActivitySourceType {
    Human,
    SystemOnboarding,
    AiPipelineCreator,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ActivitySourceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Human => serializer.serialize_str("HUMAN"),
            Self::SystemOnboarding => serializer.serialize_str("SYSTEM_ONBOARDING"),
            Self::AiPipelineCreator => serializer.serialize_str("AI_PIPELINE_CREATOR"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ActivitySourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "HUMAN" => Ok(Self::Human),
            "SYSTEM_ONBOARDING" => Ok(Self::SystemOnboarding),
            "AI_PIPELINE_CREATOR" => Ok(Self::AiPipelineCreator),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ActivitySourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Human => write!(f, "HUMAN"),
            Self::SystemOnboarding => write!(f, "SYSTEM_ONBOARDING"),
            Self::AiPipelineCreator => write!(f, "AI_PIPELINE_CREATOR"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
