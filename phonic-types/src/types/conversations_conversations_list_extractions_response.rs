pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationsListExtractionsResponse {
    #[serde(default)]
    pub extractions: Vec<ConversationExtraction>,
}

impl ConversationsListExtractionsResponse {
    pub fn builder() -> ConversationsListExtractionsResponseBuilder {
        <ConversationsListExtractionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsListExtractionsResponseBuilder {
    extractions: Option<Vec<ConversationExtraction>>,
}

impl ConversationsListExtractionsResponseBuilder {
    pub fn extractions(mut self, value: Vec<ConversationExtraction>) -> Self {
        self.extractions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsListExtractionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`extractions`](ConversationsListExtractionsResponseBuilder::extractions)
    pub fn build(self) -> Result<ConversationsListExtractionsResponse, BuildError> {
        Ok(ConversationsListExtractionsResponse {
            extractions: self.extractions.ok_or_else(|| BuildError::missing_field("extractions"))?,
        })
    }
}
