pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkflowTheme {
    Default,
    Streak,
    RainbowSaturated,
    RainbowFaded,
    TrafficLightSaturated,
    TrafficLightFaded,
    LakesideSaturated,
    LakesideFaded,
    SunsetSaturated,
    SunsetFaded,
    SummerSaturated,
    SummerFaded,
    HalloweenSaturated,
    HalloweenFaded,
    Greyscale,
    Bluegrey,
    Red,
    Yelloworange,
    Green,
    Teal,
    Cyan,
    Blue,
    Indigo,
    Deeppurple,
    Violet,
    Pink,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WorkflowTheme {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Default => serializer.serialize_str("DEFAULT"),
            Self::Streak => serializer.serialize_str("STREAK"),
            Self::RainbowSaturated => serializer.serialize_str("RAINBOW_SATURATED"),
            Self::RainbowFaded => serializer.serialize_str("RAINBOW_FADED"),
            Self::TrafficLightSaturated => serializer.serialize_str("TRAFFIC_LIGHT_SATURATED"),
            Self::TrafficLightFaded => serializer.serialize_str("TRAFFIC_LIGHT_FADED"),
            Self::LakesideSaturated => serializer.serialize_str("LAKESIDE_SATURATED"),
            Self::LakesideFaded => serializer.serialize_str("LAKESIDE_FADED"),
            Self::SunsetSaturated => serializer.serialize_str("SUNSET_SATURATED"),
            Self::SunsetFaded => serializer.serialize_str("SUNSET_FADED"),
            Self::SummerSaturated => serializer.serialize_str("SUMMER_SATURATED"),
            Self::SummerFaded => serializer.serialize_str("SUMMER_FADED"),
            Self::HalloweenSaturated => serializer.serialize_str("HALLOWEEN_SATURATED"),
            Self::HalloweenFaded => serializer.serialize_str("HALLOWEEN_FADED"),
            Self::Greyscale => serializer.serialize_str("GREYSCALE"),
            Self::Bluegrey => serializer.serialize_str("BLUEGREY"),
            Self::Red => serializer.serialize_str("RED"),
            Self::Yelloworange => serializer.serialize_str("YELLOWORANGE"),
            Self::Green => serializer.serialize_str("GREEN"),
            Self::Teal => serializer.serialize_str("TEAL"),
            Self::Cyan => serializer.serialize_str("CYAN"),
            Self::Blue => serializer.serialize_str("BLUE"),
            Self::Indigo => serializer.serialize_str("INDIGO"),
            Self::Deeppurple => serializer.serialize_str("DEEPPURPLE"),
            Self::Violet => serializer.serialize_str("VIOLET"),
            Self::Pink => serializer.serialize_str("PINK"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WorkflowTheme {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "DEFAULT" => Ok(Self::Default),
            "STREAK" => Ok(Self::Streak),
            "RAINBOW_SATURATED" => Ok(Self::RainbowSaturated),
            "RAINBOW_FADED" => Ok(Self::RainbowFaded),
            "TRAFFIC_LIGHT_SATURATED" => Ok(Self::TrafficLightSaturated),
            "TRAFFIC_LIGHT_FADED" => Ok(Self::TrafficLightFaded),
            "LAKESIDE_SATURATED" => Ok(Self::LakesideSaturated),
            "LAKESIDE_FADED" => Ok(Self::LakesideFaded),
            "SUNSET_SATURATED" => Ok(Self::SunsetSaturated),
            "SUNSET_FADED" => Ok(Self::SunsetFaded),
            "SUMMER_SATURATED" => Ok(Self::SummerSaturated),
            "SUMMER_FADED" => Ok(Self::SummerFaded),
            "HALLOWEEN_SATURATED" => Ok(Self::HalloweenSaturated),
            "HALLOWEEN_FADED" => Ok(Self::HalloweenFaded),
            "GREYSCALE" => Ok(Self::Greyscale),
            "BLUEGREY" => Ok(Self::Bluegrey),
            "RED" => Ok(Self::Red),
            "YELLOWORANGE" => Ok(Self::Yelloworange),
            "GREEN" => Ok(Self::Green),
            "TEAL" => Ok(Self::Teal),
            "CYAN" => Ok(Self::Cyan),
            "BLUE" => Ok(Self::Blue),
            "INDIGO" => Ok(Self::Indigo),
            "DEEPPURPLE" => Ok(Self::Deeppurple),
            "VIOLET" => Ok(Self::Violet),
            "PINK" => Ok(Self::Pink),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WorkflowTheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "DEFAULT"),
            Self::Streak => write!(f, "STREAK"),
            Self::RainbowSaturated => write!(f, "RAINBOW_SATURATED"),
            Self::RainbowFaded => write!(f, "RAINBOW_FADED"),
            Self::TrafficLightSaturated => write!(f, "TRAFFIC_LIGHT_SATURATED"),
            Self::TrafficLightFaded => write!(f, "TRAFFIC_LIGHT_FADED"),
            Self::LakesideSaturated => write!(f, "LAKESIDE_SATURATED"),
            Self::LakesideFaded => write!(f, "LAKESIDE_FADED"),
            Self::SunsetSaturated => write!(f, "SUNSET_SATURATED"),
            Self::SunsetFaded => write!(f, "SUNSET_FADED"),
            Self::SummerSaturated => write!(f, "SUMMER_SATURATED"),
            Self::SummerFaded => write!(f, "SUMMER_FADED"),
            Self::HalloweenSaturated => write!(f, "HALLOWEEN_SATURATED"),
            Self::HalloweenFaded => write!(f, "HALLOWEEN_FADED"),
            Self::Greyscale => write!(f, "GREYSCALE"),
            Self::Bluegrey => write!(f, "BLUEGREY"),
            Self::Red => write!(f, "RED"),
            Self::Yelloworange => write!(f, "YELLOWORANGE"),
            Self::Green => write!(f, "GREEN"),
            Self::Teal => write!(f, "TEAL"),
            Self::Cyan => write!(f, "CYAN"),
            Self::Blue => write!(f, "BLUE"),
            Self::Indigo => write!(f, "INDIGO"),
            Self::Deeppurple => write!(f, "DEEPPURPLE"),
            Self::Violet => write!(f, "VIOLET"),
            Self::Pink => write!(f, "PINK"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
