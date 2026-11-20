pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The type of tool. Each type accepts only its own fields, and a field from another type is rejected with a 400 error. Most execution flags are for custom_webhook and custom_websocket tools only. built_in_transfer_to_agent accepts only require_speech_before_tool_call, built_in_transfer_to_phone_number accepts require_speech_before_tool_call and uninterruptible, and the built_in_natural_conversation_ending, built_in_keypad_input, and built_in_choose_not_to_respond types accept none of them. Leave out any field your type doesn't accept; the field's presence is rejected, so sending it as false or null also fails. See [which tool types accept each flag](https://docs.phonic.ai/docs/build/tools/how_tools_execute#which-tool-types-accept-each-flag).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateToolRequestType {
    CustomContext,
    CustomWebhook,
    CustomWebsocket,
    BuiltInTransferToPhoneNumber,
    BuiltInTransferToAgent,
    BuiltInNaturalConversationEnding,
    BuiltInKeypadInput,
    BuiltInChooseNotToRespond,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateToolRequestType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CustomContext => serializer.serialize_str("custom_context"),
            Self::CustomWebhook => serializer.serialize_str("custom_webhook"),
            Self::CustomWebsocket => serializer.serialize_str("custom_websocket"),
            Self::BuiltInTransferToPhoneNumber => serializer.serialize_str("built_in_transfer_to_phone_number"),
            Self::BuiltInTransferToAgent => serializer.serialize_str("built_in_transfer_to_agent"),
            Self::BuiltInNaturalConversationEnding => serializer.serialize_str("built_in_natural_conversation_ending"),
            Self::BuiltInKeypadInput => serializer.serialize_str("built_in_keypad_input"),
            Self::BuiltInChooseNotToRespond => serializer.serialize_str("built_in_choose_not_to_respond"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateToolRequestType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "custom_context" => Ok(Self::CustomContext),
            "custom_webhook" => Ok(Self::CustomWebhook),
            "custom_websocket" => Ok(Self::CustomWebsocket),
            "built_in_transfer_to_phone_number" => Ok(Self::BuiltInTransferToPhoneNumber),
            "built_in_transfer_to_agent" => Ok(Self::BuiltInTransferToAgent),
            "built_in_natural_conversation_ending" => Ok(Self::BuiltInNaturalConversationEnding),
            "built_in_keypad_input" => Ok(Self::BuiltInKeypadInput),
            "built_in_choose_not_to_respond" => Ok(Self::BuiltInChooseNotToRespond),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateToolRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CustomContext => write!(f, "custom_context"),
            Self::CustomWebhook => write!(f, "custom_webhook"),
            Self::CustomWebsocket => write!(f, "custom_websocket"),
            Self::BuiltInTransferToPhoneNumber => write!(f, "built_in_transfer_to_phone_number"),
            Self::BuiltInTransferToAgent => write!(f, "built_in_transfer_to_agent"),
            Self::BuiltInNaturalConversationEnding => write!(f, "built_in_natural_conversation_ending"),
            Self::BuiltInKeypadInput => write!(f, "built_in_keypad_input"),
            Self::BuiltInChooseNotToRespond => write!(f, "built_in_choose_not_to_respond"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
