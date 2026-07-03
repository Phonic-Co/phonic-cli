pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationEvalPrompt {
    /// The evaluation prompt ID.
    #[serde(default)]
    pub id: String,
    /// The evaluation prompt name.
    #[serde(default)]
    pub name: String,
    /// The evaluation prompt text.
    #[serde(default)]
    pub prompt: String,
}

impl ConversationEvalPrompt {
    pub fn builder() -> ConversationEvalPromptBuilder {
        <ConversationEvalPromptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationEvalPromptBuilder {
    id: Option<String>,
    name: Option<String>,
    prompt: Option<String>,
}

impl ConversationEvalPromptBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationEvalPrompt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationEvalPromptBuilder::id)
    /// - [`name`](ConversationEvalPromptBuilder::name)
    /// - [`prompt`](ConversationEvalPromptBuilder::prompt)
    pub fn build(self) -> Result<ConversationEvalPrompt, BuildError> {
        Ok(ConversationEvalPrompt {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
        })
    }
}
