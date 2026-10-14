pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An assistant turn. Must carry at least one of `text`, `tool_calls`, or `action`. `tool_calls` and `action` cannot both be present.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesAssistantMessage {
    pub role: String,
    /// What the assistant said. Empty when the turn only made tool calls or only took an action.
    #[serde(default)]
    pub text: String,
    /// Tool calls the assistant made on this turn. Each one must be followed immediately by its matching `tool_call_output` item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ResponsesToolCall>>,
    /// The action the assistant took on this turn. Cannot be combined with `tool_calls`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<ResponsesAction>,
}

impl ResponsesAssistantMessage {
    pub fn builder() -> ResponsesAssistantMessageBuilder {
        <ResponsesAssistantMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesAssistantMessageBuilder {
    role: Option<String>,
    text: Option<String>,
    tool_calls: Option<Vec<ResponsesToolCall>>,
    action: Option<ResponsesAction>,
}

impl ResponsesAssistantMessageBuilder {
    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn tool_calls(mut self, value: Vec<ResponsesToolCall>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    pub fn action(mut self, value: ResponsesAction) -> Self {
        self.action = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesAssistantMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](ResponsesAssistantMessageBuilder::role)
    /// - [`text`](ResponsesAssistantMessageBuilder::text)
    pub fn build(self) -> Result<ResponsesAssistantMessage, BuildError> {
        Ok(ResponsesAssistantMessage {
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            tool_calls: self.tool_calls,
            action: self.action,
        })
    }
}
