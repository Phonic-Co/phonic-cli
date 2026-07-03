pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateConversationTokenRequest {
    /// ID of the agent the conversation token is scoped to.
    #[serde(default)]
    pub agent_id: String,
    /// Time-to-live for the conversation token in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<i64>,
}

impl CreateConversationTokenRequest {
    pub fn builder() -> CreateConversationTokenRequestBuilder {
        <CreateConversationTokenRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConversationTokenRequestBuilder {
    agent_id: Option<String>,
    ttl_seconds: Option<i64>,
}

impl CreateConversationTokenRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn ttl_seconds(mut self, value: i64) -> Self {
        self.ttl_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateConversationTokenRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](CreateConversationTokenRequestBuilder::agent_id)
    pub fn build(self) -> Result<CreateConversationTokenRequest, BuildError> {
        Ok(CreateConversationTokenRequest {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
            ttl_seconds: self.ttl_seconds,
        })
    }
}

