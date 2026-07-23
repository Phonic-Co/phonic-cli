pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The name of the built-in tool.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltInToolDefinitionName {
    KeypadInput,
    NaturalConversationEnding,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BuiltInToolDefinitionName {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::KeypadInput => serializer.serialize_str("keypad_input"),
            Self::NaturalConversationEnding => serializer.serialize_str("natural_conversation_ending"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BuiltInToolDefinitionName {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "keypad_input" => Ok(Self::KeypadInput),
            "natural_conversation_ending" => Ok(Self::NaturalConversationEnding),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BuiltInToolDefinitionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeypadInput => write!(f, "keypad_input"),
            Self::NaturalConversationEnding => write!(f, "natural_conversation_ending"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
