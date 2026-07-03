pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReadyToStartConversationPayload {
    pub r#type: String,
}

impl ReadyToStartConversationPayload {
    pub fn builder() -> ReadyToStartConversationPayloadBuilder {
        <ReadyToStartConversationPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadyToStartConversationPayloadBuilder {
    r#type: Option<String>,
}

impl ReadyToStartConversationPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReadyToStartConversationPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ReadyToStartConversationPayloadBuilder::r#type)
    pub fn build(self) -> Result<ReadyToStartConversationPayload, BuildError> {
        Ok(ReadyToStartConversationPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
