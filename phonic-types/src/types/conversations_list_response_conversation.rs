pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ConversationsListResponseConversation {
    pub conversation: Conversation,
}

impl ConversationsListResponseConversation {
    pub fn builder() -> ConversationsListResponseConversationBuilder {
        <ConversationsListResponseConversationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsListResponseConversationBuilder {
    conversation: Option<Conversation>,
}

impl ConversationsListResponseConversationBuilder {
    pub fn conversation(mut self, value: Conversation) -> Self {
        self.conversation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsListResponseConversation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation`](ConversationsListResponseConversationBuilder::conversation)
    pub fn build(self) -> Result<ConversationsListResponseConversation, BuildError> {
        Ok(ConversationsListResponseConversation {
            conversation: self.conversation.ok_or_else(|| BuildError::missing_field("conversation"))?,
        })
    }
}
