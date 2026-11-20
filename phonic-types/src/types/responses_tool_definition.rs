pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A tool the assistant may call, defined inline for this request only.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesToolDefinition {
    /// The tool name.
    #[serde(default)]
    pub name: String,
    /// What the tool does. The model uses this to decide when to call it.
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub parameters: ResponsesToolParameters,
    /// When true, forces the agent to speak before executing the tool. Note this defaults to true here, unlike on the tools API, so a simulated turn matches the default a custom tool runs with in a live call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_speech_before_tool_call: Option<bool>,
    /// When true, waits for the agent's speech to finish before calling the tool. Accepted for parity with live tools; it has no effect on generated responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_speech_before_tool_call: Option<bool>,
    /// When true, prevents the agent from speaking after executing the tool. Applied on the request that resumes the turn, the one carrying the `tool_call_output`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbid_speech_after_tool_call: Option<bool>,
    /// When true, forbids the agent from calling the tool right after it has spoken. The call is dropped and the speech is kept.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbid_tool_call_after_speech: Option<bool>,
    /// When true, allows the agent to call another tool after this tool. Applied on the request that resumes the turn, the one carrying the `tool_call_output`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_tool_chaining: Option<bool>,
}

impl ResponsesToolDefinition {
    pub fn builder() -> ResponsesToolDefinitionBuilder {
        <ResponsesToolDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesToolDefinitionBuilder {
    name: Option<String>,
    description: Option<String>,
    parameters: Option<ResponsesToolParameters>,
    require_speech_before_tool_call: Option<bool>,
    wait_for_speech_before_tool_call: Option<bool>,
    forbid_speech_after_tool_call: Option<bool>,
    forbid_tool_call_after_speech: Option<bool>,
    allow_tool_chaining: Option<bool>,
}

impl ResponsesToolDefinitionBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn parameters(mut self, value: ResponsesToolParameters) -> Self {
        self.parameters = Some(value);
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

    pub fn forbid_tool_call_after_speech(mut self, value: bool) -> Self {
        self.forbid_tool_call_after_speech = Some(value);
        self
    }

    pub fn allow_tool_chaining(mut self, value: bool) -> Self {
        self.allow_tool_chaining = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesToolDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ResponsesToolDefinitionBuilder::name)
    /// - [`description`](ResponsesToolDefinitionBuilder::description)
    /// - [`parameters`](ResponsesToolDefinitionBuilder::parameters)
    pub fn build(self) -> Result<ResponsesToolDefinition, BuildError> {
        Ok(ResponsesToolDefinition {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            parameters: self.parameters.ok_or_else(|| BuildError::missing_field("parameters"))?,
            require_speech_before_tool_call: self.require_speech_before_tool_call,
            wait_for_speech_before_tool_call: self.wait_for_speech_before_tool_call,
            forbid_speech_after_tool_call: self.forbid_speech_after_tool_call,
            forbid_tool_call_after_speech: self.forbid_tool_call_after_speech,
            allow_tool_chaining: self.allow_tool_chaining,
        })
    }
}
