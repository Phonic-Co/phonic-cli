pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EvaluateConversationRequest {
    /// ID of the evaluation prompt to use.
    #[serde(default)]
    pub prompt_id: String,
}

impl EvaluateConversationRequest {
    pub fn builder() -> EvaluateConversationRequestBuilder {
        <EvaluateConversationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluateConversationRequestBuilder {
    prompt_id: Option<String>,
}

impl EvaluateConversationRequestBuilder {
    pub fn prompt_id(mut self, value: impl Into<String>) -> Self {
        self.prompt_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EvaluateConversationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt_id`](EvaluateConversationRequestBuilder::prompt_id)
    pub fn build(self) -> Result<EvaluateConversationRequest, BuildError> {
        Ok(EvaluateConversationRequest {
            prompt_id: self.prompt_id.ok_or_else(|| BuildError::missing_field("prompt_id"))?,
        })
    }
}

