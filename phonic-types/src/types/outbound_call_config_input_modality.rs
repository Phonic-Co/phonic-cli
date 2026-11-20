pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How the user talks to the agent. `"speech"` takes user audio; `"text"` takes typed user turns sent with `generate_reply` and `user_message`. Must match `output_modality`. Phone calls only support `"speech"`. Defaults to the agent's `input_modality`, or `"speech"`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OutboundCallConfigInputModality {
    Speech,
    Text,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for OutboundCallConfigInputModality {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Speech => serializer.serialize_str("speech"),
            Self::Text => serializer.serialize_str("text"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for OutboundCallConfigInputModality {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "speech" => Ok(Self::Speech),
            "text" => Ok(Self::Text),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for OutboundCallConfigInputModality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Speech => write!(f, "speech"),
            Self::Text => write!(f, "text"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
