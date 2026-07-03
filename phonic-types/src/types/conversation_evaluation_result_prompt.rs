pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The evaluation prompt information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationEvaluationResultPrompt {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl ConversationEvaluationResultPrompt {
    pub fn builder() -> ConversationEvaluationResultPromptBuilder {
        <ConversationEvaluationResultPromptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationEvaluationResultPromptBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ConversationEvaluationResultPromptBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationEvaluationResultPrompt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationEvaluationResultPromptBuilder::id)
    /// - [`name`](ConversationEvaluationResultPromptBuilder::name)
    pub fn build(self) -> Result<ConversationEvaluationResultPrompt, BuildError> {
        Ok(ConversationEvaluationResultPrompt {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
