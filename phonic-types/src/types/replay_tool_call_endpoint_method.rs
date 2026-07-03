pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// HTTP method for webhook tool calls.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReplayToolCallEndpointMethod {
    Get,
    Post,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ReplayToolCallEndpointMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Get => serializer.serialize_str("GET"),
            Self::Post => serializer.serialize_str("POST"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ReplayToolCallEndpointMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "GET" => Ok(Self::Get),
            "POST" => Ok(Self::Post),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ReplayToolCallEndpointMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
