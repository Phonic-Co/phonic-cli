pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of tool.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolType {
    CustomContext,
    CustomWebhook,
    CustomWebsocket,
    BuiltInTransferToPhoneNumber,
    BuiltInTransferToAgent,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CustomContext => serializer.serialize_str("custom_context"),
            Self::CustomWebhook => serializer.serialize_str("custom_webhook"),
            Self::CustomWebsocket => serializer.serialize_str("custom_websocket"),
            Self::BuiltInTransferToPhoneNumber => serializer.serialize_str("built_in_transfer_to_phone_number"),
            Self::BuiltInTransferToAgent => serializer.serialize_str("built_in_transfer_to_agent"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "custom_context" => Ok(Self::CustomContext),
            "custom_webhook" => Ok(Self::CustomWebhook),
            "custom_websocket" => Ok(Self::CustomWebsocket),
            "built_in_transfer_to_phone_number" => Ok(Self::BuiltInTransferToPhoneNumber),
            "built_in_transfer_to_agent" => Ok(Self::BuiltInTransferToAgent),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CustomContext => write!(f, "custom_context"),
            Self::CustomWebhook => write!(f, "custom_webhook"),
            Self::CustomWebsocket => write!(f, "custom_websocket"),
            Self::BuiltInTransferToPhoneNumber => write!(f, "built_in_transfer_to_phone_number"),
            Self::BuiltInTransferToAgent => write!(f, "built_in_transfer_to_agent"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
