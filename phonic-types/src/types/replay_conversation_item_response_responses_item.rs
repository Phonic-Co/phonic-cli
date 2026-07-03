pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReplayConversationItemResponseResponsesItem {
    /// The text the LLM produced for this replay response.
    #[serde(default)]
    pub text: String,
    /// Tool calls the LLM would invoke for this replay response - note that the tools will not actually be called during replay.
    #[serde(default)]
    pub tool_calls: Vec<ReplayToolCall>,
}

impl ReplayConversationItemResponseResponsesItem {
    pub fn builder() -> ReplayConversationItemResponseResponsesItemBuilder {
        <ReplayConversationItemResponseResponsesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayConversationItemResponseResponsesItemBuilder {
    text: Option<String>,
    tool_calls: Option<Vec<ReplayToolCall>>,
}

impl ReplayConversationItemResponseResponsesItemBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn tool_calls(mut self, value: Vec<ReplayToolCall>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayConversationItemResponseResponsesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](ReplayConversationItemResponseResponsesItemBuilder::text)
    /// - [`tool_calls`](ReplayConversationItemResponseResponsesItemBuilder::tool_calls)
    pub fn build(self) -> Result<ReplayConversationItemResponseResponsesItem, BuildError> {
        Ok(ReplayConversationItemResponseResponsesItem {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            tool_calls: self.tool_calls.ok_or_else(|| BuildError::missing_field("tool_calls"))?,
        })
    }
}
