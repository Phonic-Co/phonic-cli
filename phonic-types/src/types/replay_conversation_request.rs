pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplayConversationRequest {
    /// Name of the agent to replay the conversation with. Defaults to the agent originally associated with the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
}

impl ReplayConversationRequest {
    pub fn builder() -> ReplayConversationRequestBuilder {
        <ReplayConversationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayConversationRequestBuilder {
    agent: Option<String>,
}

impl ReplayConversationRequestBuilder {
    pub fn agent(mut self, value: impl Into<String>) -> Self {
        self.agent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReplayConversationRequest`].
    pub fn build(self) -> Result<ReplayConversationRequest, BuildError> {
        Ok(ReplayConversationRequest {
            agent: self.agent,
        })
    }
}

