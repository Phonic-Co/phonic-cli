pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsListEvaluationsResponse {
    #[serde(default)]
    pub evals: Vec<ConversationEvaluationResult>,
}

impl ConversationsListEvaluationsResponse {
    pub fn builder() -> ConversationsListEvaluationsResponseBuilder {
        <ConversationsListEvaluationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsListEvaluationsResponseBuilder {
    evals: Option<Vec<ConversationEvaluationResult>>,
}

impl ConversationsListEvaluationsResponseBuilder {
    pub fn evals(mut self, value: Vec<ConversationEvaluationResult>) -> Self {
        self.evals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsListEvaluationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evals`](ConversationsListEvaluationsResponseBuilder::evals)
    pub fn build(self) -> Result<ConversationsListEvaluationsResponse, BuildError> {
        Ok(ConversationsListEvaluationsResponse {
            evals: self.evals.ok_or_else(|| BuildError::missing_field("evals"))?,
        })
    }
}
