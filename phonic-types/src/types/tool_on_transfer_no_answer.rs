pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What happens when the transfer target does not answer before the ring timeout. `return_to_assistant` hands control back to the agent. `keep_retrying` keeps the caller on the line and re-dials the target until it answers, the caller hangs up, or a retry cap is reached, then returns to the assistant. `keep_retrying` only applies to bridged transfers (keep_listening true). Only returned for built_in_transfer_to_phone_number tools.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolOnTransferNoAnswer {
    ReturnToAssistant,
    KeepRetrying,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolOnTransferNoAnswer {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ReturnToAssistant => serializer.serialize_str("return_to_assistant"),
            Self::KeepRetrying => serializer.serialize_str("keep_retrying"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolOnTransferNoAnswer {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "return_to_assistant" => Ok(Self::ReturnToAssistant),
            "keep_retrying" => Ok(Self::KeepRetrying),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolOnTransferNoAnswer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReturnToAssistant => write!(f, "return_to_assistant"),
            Self::KeepRetrying => write!(f, "keep_retrying"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
