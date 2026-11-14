pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplayConversationItemRequest {
    /// Optional model override for this replay. Omit it to use the original model. Select a model available to your workspace. Phonic 0.5 (phonic-0.5) is deprecated and remains available only to customers who used it before Phonic 1 launched. The conversation's saved intelligence level is retained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ReplayConversationItemRequestModel>,
    /// The system prompt to use when generating replay responses. Use this to test prompt changes against this conversation turn. Omit it to replay the turn with the system prompt it originally ran with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
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
    model: Option<ReplayConversationItemRequestModel>,
    system_prompt: Option<String>,
    num_responses: Option<i64>,
}

impl ReplayConversationItemRequestBuilder {
    pub fn model(mut self, value: ReplayConversationItemRequestModel) -> Self {
        self.model = Some(value);
        self
    }

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    pub fn num_responses(mut self, value: i64) -> Self {
        self.num_responses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayConversationItemRequest`].
    pub fn build(self) -> Result<ReplayConversationItemRequest, BuildError> {
        Ok(ReplayConversationItemRequest {
            model: self.model,
            system_prompt: self.system_prompt,
            num_responses: self.num_responses,
        })
    }
}

