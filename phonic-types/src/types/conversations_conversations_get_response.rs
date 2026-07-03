pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationsGetResponse {
    pub conversation: Conversation,
}

impl ConversationsGetResponse {
    pub fn builder() -> ConversationsGetResponseBuilder {
        <ConversationsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsGetResponseBuilder {
    conversation: Option<Conversation>,
}

impl ConversationsGetResponseBuilder {
    pub fn conversation(mut self, value: Conversation) -> Self {
        self.conversation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation`](ConversationsGetResponseBuilder::conversation)
    pub fn build(self) -> Result<ConversationsGetResponse, BuildError> {
        Ok(ConversationsGetResponse {
            conversation: self.conversation.ok_or_else(|| BuildError::missing_field("conversation"))?,
        })
    }
}
