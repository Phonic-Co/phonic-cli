pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationEval {
    /// The evaluation ID.
    #[serde(default)]
    pub id: String,
    /// The ID of the conversation that was evaluated.
    #[serde(default)]
    pub conversation_id: String,
    #[serde(default)]
    pub prompt: ConversationEvalPromptInfo,
    /// The evaluation result.
    pub result: ConversationEvalResult,
    /// When the evaluation was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ConversationEval {
    pub fn builder() -> ConversationEvalBuilder {
        <ConversationEvalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationEvalBuilder {
    id: Option<String>,
    conversation_id: Option<String>,
    prompt: Option<ConversationEvalPromptInfo>,
    result: Option<ConversationEvalResult>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ConversationEvalBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: ConversationEvalPromptInfo) -> Self {
        self.prompt = Some(value);
        self
    }

    pub fn result(mut self, value: ConversationEvalResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationEval`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationEvalBuilder::id)
    /// - [`conversation_id`](ConversationEvalBuilder::conversation_id)
    /// - [`prompt`](ConversationEvalBuilder::prompt)
    /// - [`result`](ConversationEvalBuilder::result)
    /// - [`created_at`](ConversationEvalBuilder::created_at)
    pub fn build(self) -> Result<ConversationEval, BuildError> {
        Ok(ConversationEval {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
