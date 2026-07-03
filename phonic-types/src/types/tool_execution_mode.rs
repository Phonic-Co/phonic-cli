pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Mode of operation - sync waits for response, async continues without waiting.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolExecutionMode {
    Sync,
    Async,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolExecutionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sync => serializer.serialize_str("sync"),
            Self::Async => serializer.serialize_str("async"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolExecutionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sync" => Ok(Self::Sync),
            "async" => Ok(Self::Async),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolExecutionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sync => write!(f, "sync"),
            Self::Async => write!(f, "async"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
