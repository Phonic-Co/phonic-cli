pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Inline WebSocket tool definition for this conversation. Inline tools are not persisted to your workspace; they are executed by your connected WebSocket application when Phonic sends a `tool_call` message.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InlineWebSocketTool {
    pub r#type: String,
    pub tool_schema: OpenAiTool,
    /// Whether the assistant waits for the tool output before continuing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<InlineWebSocketToolExecutionMode>,
    /// Timeout in milliseconds for the client to send a `tool_call_output` message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_output_timeout_ms: Option<i64>,
    /// When true, forces the assistant to speak before executing the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_speech_before_tool_call: Option<bool>,
    /// When true, waits for the assistant's speech to finish before sending the tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_speech_before_tool_call: Option<bool>,
    /// When true, prevents the assistant from speaking after executing the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbid_speech_after_tool_call: Option<bool>,
    /// When true, allows the assistant to call another tool after this tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_tool_chaining: Option<bool>,
    /// For async tools, when true, the assistant waits for the response and speaks when it arrives.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_response: Option<bool>,
}

impl InlineWebSocketTool {
    pub fn builder() -> InlineWebSocketToolBuilder {
        <InlineWebSocketToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InlineWebSocketToolBuilder {
    r#type: Option<String>,
    tool_schema: Option<OpenAiTool>,
    execution_mode: Option<InlineWebSocketToolExecutionMode>,
    tool_call_output_timeout_ms: Option<i64>,
    require_speech_before_tool_call: Option<bool>,
    wait_for_speech_before_tool_call: Option<bool>,
    forbid_speech_after_tool_call: Option<bool>,
    allow_tool_chaining: Option<bool>,
    wait_for_response: Option<bool>,
}

impl InlineWebSocketToolBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn tool_schema(mut self, value: OpenAiTool) -> Self {
        self.tool_schema = Some(value);
        self
    }

    pub fn execution_mode(mut self, value: InlineWebSocketToolExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn tool_call_output_timeout_ms(mut self, value: i64) -> Self {
        self.tool_call_output_timeout_ms = Some(value);
        self
    }

    pub fn require_speech_before_tool_call(mut self, value: bool) -> Self {
        self.require_speech_before_tool_call = Some(value);
        self
    }

    pub fn wait_for_speech_before_tool_call(mut self, value: bool) -> Self {
        self.wait_for_speech_before_tool_call = Some(value);
        self
    }

    pub fn forbid_speech_after_tool_call(mut self, value: bool) -> Self {
        self.forbid_speech_after_tool_call = Some(value);
        self
    }

    pub fn allow_tool_chaining(mut self, value: bool) -> Self {
        self.allow_tool_chaining = Some(value);
        self
    }

    pub fn wait_for_response(mut self, value: bool) -> Self {
        self.wait_for_response = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InlineWebSocketTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](InlineWebSocketToolBuilder::r#type)
    /// - [`tool_schema`](InlineWebSocketToolBuilder::tool_schema)
    pub fn build(self) -> Result<InlineWebSocketTool, BuildError> {
        Ok(InlineWebSocketTool {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            tool_schema: self.tool_schema.ok_or_else(|| BuildError::missing_field("tool_schema"))?,
            execution_mode: self.execution_mode,
            tool_call_output_timeout_ms: self.tool_call_output_timeout_ms,
            require_speech_before_tool_call: self.require_speech_before_tool_call,
            wait_for_speech_before_tool_call: self.wait_for_speech_before_tool_call,
            forbid_speech_after_tool_call: self.forbid_speech_after_tool_call,
            allow_tool_chaining: self.allow_tool_chaining,
            wait_for_response: self.wait_for_response,
        })
    }
}
