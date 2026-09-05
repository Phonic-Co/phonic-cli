pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponsesInputItem {
        ResponsesUserMessage(ResponsesUserMessage),

        ResponsesAssistantMessage(ResponsesAssistantMessage),

        ResponsesToolCallOutput(ResponsesToolCallOutput),
}

impl ResponsesInputItem {
    pub fn is_responses_user_message(&self) -> bool {
        matches!(self, Self::ResponsesUserMessage(_))
    }

    pub fn is_responses_assistant_message(&self) -> bool {
        matches!(self, Self::ResponsesAssistantMessage(_))
    }

    pub fn is_responses_tool_call_output(&self) -> bool {
        matches!(self, Self::ResponsesToolCallOutput(_))
    }


    pub fn as_responses_user_message(&self) -> Option<&ResponsesUserMessage> {
        match self {
                    Self::ResponsesUserMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_responses_user_message(self) -> Option<ResponsesUserMessage> {
        match self {
                    Self::ResponsesUserMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_responses_assistant_message(&self) -> Option<&ResponsesAssistantMessage> {
        match self {
                    Self::ResponsesAssistantMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_responses_assistant_message(self) -> Option<ResponsesAssistantMessage> {
        match self {
                    Self::ResponsesAssistantMessage(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_responses_tool_call_output(&self) -> Option<&ResponsesToolCallOutput> {
        match self {
                    Self::ResponsesToolCallOutput(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_responses_tool_call_output(self) -> Option<ResponsesToolCallOutput> {
        match self {
                    Self::ResponsesToolCallOutput(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ResponsesInputItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponsesUserMessage(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ResponsesAssistantMessage(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ResponsesToolCallOutput(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
