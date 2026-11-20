pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WebhookEvent {
    ConversationEnded,
    ConversationAnalysis,
    ConversationTransferred,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for WebhookEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ConversationEnded => serializer.serialize_str("conversation.ended"),
            Self::ConversationAnalysis => serializer.serialize_str("conversation.analysis"),
            Self::ConversationTransferred => serializer.serialize_str("conversation.transferred"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for WebhookEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "conversation.ended" => Ok(Self::ConversationEnded),
            "conversation.analysis" => Ok(Self::ConversationAnalysis),
            "conversation.transferred" => Ok(Self::ConversationTransferred),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for WebhookEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationEnded => write!(f, "conversation.ended"),
            Self::ConversationAnalysis => write!(f, "conversation.analysis"),
            Self::ConversationTransferred => write!(f, "conversation.transferred"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
