pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConversationsListResponse {
        ConversationsListResponseConversations(ConversationsListResponseConversations),

        ConversationsListResponseConversation(ConversationsListResponseConversation),
}

impl ConversationsListResponse {
    pub fn is_conversations_list_response_conversations(&self) -> bool {
        matches!(self, Self::ConversationsListResponseConversations(_))
    }

    pub fn is_conversations_list_response_conversation(&self) -> bool {
        matches!(self, Self::ConversationsListResponseConversation(_))
    }


    pub fn as_conversations_list_response_conversations(&self) -> Option<&ConversationsListResponseConversations> {
        match self {
                    Self::ConversationsListResponseConversations(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conversations_list_response_conversations(self) -> Option<ConversationsListResponseConversations> {
        match self {
                    Self::ConversationsListResponseConversations(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_conversations_list_response_conversation(&self) -> Option<&ConversationsListResponseConversation> {
        match self {
                    Self::ConversationsListResponseConversation(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_conversations_list_response_conversation(self) -> Option<ConversationsListResponseConversation> {
        match self {
                    Self::ConversationsListResponseConversation(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ConversationsListResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversationsListResponseConversations(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConversationsListResponseConversation(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
