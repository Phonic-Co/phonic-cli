pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConversationsEvaluateResponse {
    pub evaluation: ConversationsEvaluateResponseEvaluation,
}

impl ConversationsEvaluateResponse {
    pub fn builder() -> ConversationsEvaluateResponseBuilder {
        <ConversationsEvaluateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsEvaluateResponseBuilder {
    evaluation: Option<ConversationsEvaluateResponseEvaluation>,
}

impl ConversationsEvaluateResponseBuilder {
    pub fn evaluation(mut self, value: ConversationsEvaluateResponseEvaluation) -> Self {
        self.evaluation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsEvaluateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evaluation`](ConversationsEvaluateResponseBuilder::evaluation)
    pub fn build(self) -> Result<ConversationsEvaluateResponse, BuildError> {
        Ok(ConversationsEvaluateResponse {
            evaluation: self.evaluation.ok_or_else(|| BuildError::missing_field("evaluation"))?,
        })
    }
}
