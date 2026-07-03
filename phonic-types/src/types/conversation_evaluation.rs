pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationEvaluation {
    /// The evaluation result.
    pub result: ConversationEvaluationResult,
}

impl ConversationEvaluation {
    pub fn builder() -> ConversationEvaluationBuilder {
        <ConversationEvaluationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationEvaluationBuilder {
    result: Option<ConversationEvaluationResult>,
}

impl ConversationEvaluationBuilder {
    pub fn result(mut self, value: ConversationEvaluationResult) -> Self {
        self.result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationEvaluation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result`](ConversationEvaluationBuilder::result)
    pub fn build(self) -> Result<ConversationEvaluation, BuildError> {
        Ok(ConversationEvaluation {
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
        })
    }
}
