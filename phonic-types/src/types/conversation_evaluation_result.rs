pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationEvaluationResult {
    /// The evaluation ID.
    #[serde(default)]
    pub id: String,
    /// The evaluation result.
    pub result: ConversationEvaluationResultResult,
    /// The evaluation prompt information.
    #[serde(default)]
    pub prompt: ConversationEvaluationResultPrompt,
    /// When the evaluation was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ConversationEvaluationResult {
    pub fn builder() -> ConversationEvaluationResultBuilder {
        <ConversationEvaluationResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationEvaluationResultBuilder {
    id: Option<String>,
    result: Option<ConversationEvaluationResultResult>,
    prompt: Option<ConversationEvaluationResultPrompt>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ConversationEvaluationResultBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn result(mut self, value: ConversationEvaluationResultResult) -> Self {
        self.result = Some(value);
        self
    }

    pub fn prompt(mut self, value: ConversationEvaluationResultPrompt) -> Self {
        self.prompt = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationEvaluationResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationEvaluationResultBuilder::id)
    /// - [`result`](ConversationEvaluationResultBuilder::result)
    /// - [`prompt`](ConversationEvaluationResultBuilder::prompt)
    /// - [`created_at`](ConversationEvaluationResultBuilder::created_at)
    pub fn build(self) -> Result<ConversationEvaluationResult, BuildError> {
        Ok(ConversationEvaluationResult {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
