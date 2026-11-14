pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The role of the conversation turn the tool call was attached to. A tool call made while the user is still speaking is attached to the user's turn.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolCallOutputProcessedPayloadConversationItemRole {
    User,
    Assistant,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolCallOutputProcessedPayloadConversationItemRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::User => serializer.serialize_str("user"),
            Self::Assistant => serializer.serialize_str("assistant"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolCallOutputProcessedPayloadConversationItemRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user" => Ok(Self::User),
            "assistant" => Ok(Self::Assistant),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolCallOutputProcessedPayloadConversationItemRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::Assistant => write!(f, "assistant"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
