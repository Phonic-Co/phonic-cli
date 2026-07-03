pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The background noise type. Can be "office", "call-center", "coffee-shop", or null.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateAgentRequestBackgroundNoise {
    Office,
    CallCenter,
    CoffeeShop,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateAgentRequestBackgroundNoise {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Office => serializer.serialize_str("office"),
            Self::CallCenter => serializer.serialize_str("call-center"),
            Self::CoffeeShop => serializer.serialize_str("coffee-shop"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateAgentRequestBackgroundNoise {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "office" => Ok(Self::Office),
            "call-center" => Ok(Self::CallCenter),
            "coffee-shop" => Ok(Self::CoffeeShop),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateAgentRequestBackgroundNoise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Office => write!(f, "office"),
            Self::CallCenter => write!(f, "call-center"),
            Self::CoffeeShop => write!(f, "coffee-shop"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
