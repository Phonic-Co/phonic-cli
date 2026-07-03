pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateConversationEvalPromptRequest {
    /// A useful name for referring to this prompt.
    #[serde(default)]
    pub name: String,
    /// Actual evaluation prompt text to evaluate conversations with.
    #[serde(default)]
    pub prompt: String,
}

impl CreateConversationEvalPromptRequest {
    pub fn builder() -> CreateConversationEvalPromptRequestBuilder {
        <CreateConversationEvalPromptRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConversationEvalPromptRequestBuilder {
    name: Option<String>,
    prompt: Option<String>,
}

impl CreateConversationEvalPromptRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateConversationEvalPromptRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateConversationEvalPromptRequestBuilder::name)
    /// - [`prompt`](CreateConversationEvalPromptRequestBuilder::prompt)
    pub fn build(self) -> Result<CreateConversationEvalPromptRequest, BuildError> {
        Ok(CreateConversationEvalPromptRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
        })
    }
}

