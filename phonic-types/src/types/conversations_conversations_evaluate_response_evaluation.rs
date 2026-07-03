pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationsEvaluateResponseEvaluation {
    /// The evaluation result.
    pub result: ConversationsEvaluateResponseEvaluationResult,
}

impl ConversationsEvaluateResponseEvaluation {
    pub fn builder() -> ConversationsEvaluateResponseEvaluationBuilder {
        <ConversationsEvaluateResponseEvaluationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsEvaluateResponseEvaluationBuilder {
    result: Option<ConversationsEvaluateResponseEvaluationResult>,
}

impl ConversationsEvaluateResponseEvaluationBuilder {
    pub fn result(mut self, value: ConversationsEvaluateResponseEvaluationResult) -> Self {
        self.result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsEvaluateResponseEvaluation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result`](ConversationsEvaluateResponseEvaluationBuilder::result)
    pub fn build(self) -> Result<ConversationsEvaluateResponseEvaluation, BuildError> {
        Ok(ConversationsEvaluateResponseEvaluation {
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
        })
    }
}
