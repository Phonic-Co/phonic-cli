pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplayConversationItemRequest {
    /// The system prompt to use when generating replay responses. Use this to test prompt changes against this conversation turn.
    #[serde(default)]
    pub system_prompt: String,
    /// Number of alternative responses to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_responses: Option<i64>,
}

impl ReplayConversationItemRequest {
    pub fn builder() -> ReplayConversationItemRequestBuilder {
        <ReplayConversationItemRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayConversationItemRequestBuilder {
    system_prompt: Option<String>,
    num_responses: Option<i64>,
}

impl ReplayConversationItemRequestBuilder {
    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    pub fn num_responses(mut self, value: i64) -> Self {
        self.num_responses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayConversationItemRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`system_prompt`](ReplayConversationItemRequestBuilder::system_prompt)
    pub fn build(self) -> Result<ReplayConversationItemRequest, BuildError> {
        Ok(ReplayConversationItemRequest {
            system_prompt: self.system_prompt.ok_or_else(|| BuildError::missing_field("system_prompt"))?,
            num_responses: self.num_responses,
        })
    }
}

