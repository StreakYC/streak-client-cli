pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Content used as context for AI autofill. BOX_TIMELINE uses the pipeline item's timeline; WEB_CRAWLING uses content found on the web.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipelineAutofillSource {
    BoxTimeline,
    WebCrawling,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PipelineAutofillSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::BoxTimeline => serializer.serialize_str("BOX_TIMELINE"),
            Self::WebCrawling => serializer.serialize_str("WEB_CRAWLING"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PipelineAutofillSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "BOX_TIMELINE" => Ok(Self::BoxTimeline),
            "WEB_CRAWLING" => Ok(Self::WebCrawling),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PipelineAutofillSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BoxTimeline => write!(f, "BOX_TIMELINE"),
            Self::WebCrawling => write!(f, "WEB_CRAWLING"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
