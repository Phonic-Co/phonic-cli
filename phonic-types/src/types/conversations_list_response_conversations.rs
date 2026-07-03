pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationsListResponseConversations {
    #[serde(default)]
    pub conversations: Vec<Conversation>,
    #[serde(default)]
    pub pagination: ConversationsListResponseConversationsPagination,
}

impl ConversationsListResponseConversations {
    pub fn builder() -> ConversationsListResponseConversationsBuilder {
        <ConversationsListResponseConversationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsListResponseConversationsBuilder {
    conversations: Option<Vec<Conversation>>,
    pagination: Option<ConversationsListResponseConversationsPagination>,
}

impl ConversationsListResponseConversationsBuilder {
    pub fn conversations(mut self, value: Vec<Conversation>) -> Self {
        self.conversations = Some(value);
        self
    }

    pub fn pagination(mut self, value: ConversationsListResponseConversationsPagination) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsListResponseConversations`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversations`](ConversationsListResponseConversationsBuilder::conversations)
    /// - [`pagination`](ConversationsListResponseConversationsBuilder::pagination)
    pub fn build(self) -> Result<ConversationsListResponseConversations, BuildError> {
        Ok(ConversationsListResponseConversations {
            conversations: self.conversations.ok_or_else(|| BuildError::missing_field("conversations"))?,
            pagination: self.pagination.ok_or_else(|| BuildError::missing_field("pagination"))?,
        })
    }
}
