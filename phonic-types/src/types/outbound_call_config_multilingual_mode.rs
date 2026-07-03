pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// If `"auto"`, each user audio is automatically identified for the language to respond in. If `"request"`, user must request to change language (recommended). If `"initial"` the first turn user audio determines the language for the rest of the conversation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OutboundCallConfigMultilingualMode {
    Auto,
    Request,
    Initial,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OutboundCallConfigMultilingualMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Request => serializer.serialize_str("request"),
            Self::Initial => serializer.serialize_str("initial"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OutboundCallConfigMultilingualMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "request" => Ok(Self::Request),
            "initial" => Ok(Self::Initial),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OutboundCallConfigMultilingualMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Request => write!(f, "request"),
            Self::Initial => write!(f, "initial"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
