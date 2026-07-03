pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationCreatedPayload {
    pub r#type: String,
    /// ID of the created conversation
    #[serde(default)]
    pub conversation_id: String,
}

impl ConversationCreatedPayload {
    pub fn builder() -> ConversationCreatedPayloadBuilder {
        <ConversationCreatedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationCreatedPayloadBuilder {
    r#type: Option<String>,
    conversation_id: Option<String>,
}

impl ConversationCreatedPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationCreatedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ConversationCreatedPayloadBuilder::r#type)
    /// - [`conversation_id`](ConversationCreatedPayloadBuilder::conversation_id)
    pub fn build(self) -> Result<ConversationCreatedPayload, BuildError> {
        Ok(ConversationCreatedPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
        })
    }
}
