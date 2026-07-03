pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Who or what ended the conversation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationEndedBy {
    User,
    UserCanceled,
    UserValidationFailed,
    Assistant,
    AssistantSilenceLimitReached,
    ConfigurationEndpointTimedOut,
    ConfigurationEndpointError,
    ConfigurationEndpointInvalidResponse,
    Error,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationEndedBy {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::User => serializer.serialize_str("user"),
            Self::UserCanceled => serializer.serialize_str("user_canceled"),
            Self::UserValidationFailed => serializer.serialize_str("user_validation_failed"),
            Self::Assistant => serializer.serialize_str("assistant"),
            Self::AssistantSilenceLimitReached => serializer.serialize_str("assistant_silence_limit_reached"),
            Self::ConfigurationEndpointTimedOut => serializer.serialize_str("configuration_endpoint_timed_out"),
            Self::ConfigurationEndpointError => serializer.serialize_str("configuration_endpoint_error"),
            Self::ConfigurationEndpointInvalidResponse => serializer.serialize_str("configuration_endpoint_invalid_response"),
            Self::Error => serializer.serialize_str("error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationEndedBy {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "user" => Ok(Self::User),
            "user_canceled" => Ok(Self::UserCanceled),
            "user_validation_failed" => Ok(Self::UserValidationFailed),
            "assistant" => Ok(Self::Assistant),
            "assistant_silence_limit_reached" => Ok(Self::AssistantSilenceLimitReached),
            "configuration_endpoint_timed_out" => Ok(Self::ConfigurationEndpointTimedOut),
            "configuration_endpoint_error" => Ok(Self::ConfigurationEndpointError),
            "configuration_endpoint_invalid_response" => Ok(Self::ConfigurationEndpointInvalidResponse),
            "error" => Ok(Self::Error),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationEndedBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::User => write!(f, "user"),
            Self::UserCanceled => write!(f, "user_canceled"),
            Self::UserValidationFailed => write!(f, "user_validation_failed"),
            Self::Assistant => write!(f, "assistant"),
            Self::AssistantSilenceLimitReached => write!(f, "assistant_silence_limit_reached"),
            Self::ConfigurationEndpointTimedOut => write!(f, "configuration_endpoint_timed_out"),
            Self::ConfigurationEndpointError => write!(f, "configuration_endpoint_error"),
            Self::ConfigurationEndpointInvalidResponse => write!(f, "configuration_endpoint_invalid_response"),
            Self::Error => write!(f, "error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
