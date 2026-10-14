pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// For built_in_natural_conversation_ending and built_in_keypad_input tools. Whether the agent must speak before calling the tool ("required"), the model decides ("optional"), or the agent must stay silent ("suppressed"). Not used by other tool types.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateToolRequestSpeechBeforeToolCall {
    Required,
    Optional,
    Suppressed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateToolRequestSpeechBeforeToolCall {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Required => serializer.serialize_str("required"),
            Self::Optional => serializer.serialize_str("optional"),
            Self::Suppressed => serializer.serialize_str("suppressed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateToolRequestSpeechBeforeToolCall {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "required" => Ok(Self::Required),
            "optional" => Ok(Self::Optional),
            "suppressed" => Ok(Self::Suppressed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateToolRequestSpeechBeforeToolCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Required => write!(f, "required"),
            Self::Optional => write!(f, "optional"),
            Self::Suppressed => write!(f, "suppressed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
