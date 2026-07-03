pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationsGetAnalysisResponse {
    #[serde(default)]
    pub analysis: ConversationAnalysis,
}

impl ConversationsGetAnalysisResponse {
    pub fn builder() -> ConversationsGetAnalysisResponseBuilder {
        <ConversationsGetAnalysisResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsGetAnalysisResponseBuilder {
    analysis: Option<ConversationAnalysis>,
}

impl ConversationsGetAnalysisResponseBuilder {
    pub fn analysis(mut self, value: ConversationAnalysis) -> Self {
        self.analysis = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsGetAnalysisResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`analysis`](ConversationsGetAnalysisResponseBuilder::analysis)
    pub fn build(self) -> Result<ConversationsGetAnalysisResponse, BuildError> {
        Ok(ConversationsGetAnalysisResponse {
            analysis: self.analysis.ok_or_else(|| BuildError::missing_field("analysis"))?,
        })
    }
}
