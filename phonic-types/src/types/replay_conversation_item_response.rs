pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReplayConversationItemResponse {
    /// Alternative responses generated for the conversation item.
    #[serde(default)]
    pub responses: Vec<ReplayConversationItemResponseResponsesItem>,
}

impl ReplayConversationItemResponse {
    pub fn builder() -> ReplayConversationItemResponseBuilder {
        <ReplayConversationItemResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayConversationItemResponseBuilder {
    responses: Option<Vec<ReplayConversationItemResponseResponsesItem>>,
}

impl ReplayConversationItemResponseBuilder {
    pub fn responses(mut self, value: Vec<ReplayConversationItemResponseResponsesItem>) -> Self {
        self.responses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayConversationItemResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`responses`](ReplayConversationItemResponseBuilder::responses)
    pub fn build(self) -> Result<ReplayConversationItemResponse, BuildError> {
        Ok(ReplayConversationItemResponse {
            responses: self.responses.ok_or_else(|| BuildError::missing_field("responses"))?,
        })
    }
}
