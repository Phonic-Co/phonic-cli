pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OutboundCallInitiatedResponse {
    /// The ID of the created conversation.
    #[serde(default)]
    pub conversation_id: String,
    /// Always false when a call was placed.
    pub dry_run: bool,
}

impl OutboundCallInitiatedResponse {
    pub fn builder() -> OutboundCallInitiatedResponseBuilder {
        <OutboundCallInitiatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutboundCallInitiatedResponseBuilder {
    conversation_id: Option<String>,
    dry_run: Option<bool>,
}

impl OutboundCallInitiatedResponseBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OutboundCallInitiatedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](OutboundCallInitiatedResponseBuilder::conversation_id)
    /// - [`dry_run`](OutboundCallInitiatedResponseBuilder::dry_run)
    pub fn build(self) -> Result<OutboundCallInitiatedResponse, BuildError> {
        Ok(OutboundCallInitiatedResponse {
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            dry_run: self.dry_run.ok_or_else(|| BuildError::missing_field("dry_run"))?,
        })
    }
}
