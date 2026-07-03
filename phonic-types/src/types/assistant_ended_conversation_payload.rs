pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssistantEndedConversationPayload {
    pub r#type: String,
}

impl AssistantEndedConversationPayload {
    pub fn builder() -> AssistantEndedConversationPayloadBuilder {
        <AssistantEndedConversationPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssistantEndedConversationPayloadBuilder {
    r#type: Option<String>,
}

impl AssistantEndedConversationPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssistantEndedConversationPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AssistantEndedConversationPayloadBuilder::r#type)
    pub fn build(self) -> Result<AssistantEndedConversationPayload, BuildError> {
        Ok(AssistantEndedConversationPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
