pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tool {
    /// The ID of the tool.
    #[serde(default)]
    pub id: String,
    /// The name of the tool.
    #[serde(default)]
    pub name: String,
    /// Description of what the tool does.
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub project: ToolProject,
    /// The type of tool.
    pub r#type: ToolType,
    /// Mode of operation - sync waits for response, async continues without waiting.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_mode: Option<ToolExecutionMode>,
    /// Array of parameter definitions for the tool.
    #[serde(default)]
    pub parameters: Vec<ToolParameter>,
    /// HTTP method for webhook tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_method: Option<ToolEndpointMethod>,
    /// URL for webhook tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// Headers for webhook tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_headers: Option<HashMap<String, String>>,
    /// Timeout in milliseconds for webhook tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_timeout_ms: Option<i64>,
    /// Timeout in milliseconds for WebSocket tool responses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_output_timeout_ms: Option<i64>,
    /// The E.164 formatted phone number to transfer calls to. Set to null if the agent should determine the phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// DTMF digits to send after the transfer connects (e.g., "1234"). Defaults to null. Ignored when dynamic_dtmf is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtmf: Option<String>,
    /// When true, the agent determines the DTMF digits at call time (and may choose to send none); the static dtmf is ignored. Only sent when use_agent_phone_number is true (not on a SIP REFER transfer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_dtmf: Option<bool>,
    /// When true, Phonic will transfer the call using the agent's phone number. When false, Phonic will transfer the call using the phone number of the party to whom the agent is connected. This is only available for built_in_transfer_to_phone_number tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_agent_phone_number: Option<bool>,
    /// When true, Phonic will listen in and tell the user if the transfer hits voicemail. This is only available for built_in_transfer_to_phone_number tools when use_agent_phone_number is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detect_voicemail: Option<bool>,
    /// Array of agent names that the LLM can choose from when transferring. Required for built_in_transfer_to_agent tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents_to_transfer_to: Option<Vec<String>>,
    /// When true, forces the agent to speak before executing the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_speech_before_tool_call: Option<bool>,
    /// If true, the agent will wait to finish speaking before executing the tool. This is only available for custom_webhook and custom_websocket tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_speech_before_tool_call: Option<bool>,
    /// When true, forbids the agent from speaking after executing the tool. Available for custom_context, custom_webhook and custom_websocket tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forbid_speech_after_tool_call: Option<bool>,
    /// When true, allows the agent to chain and execute other tools after executing the tool. Available for custom_context, custom_webhook and custom_websocket tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_tool_chaining: Option<bool>,
    /// The agent doesn't typically wait for the response of async custom_websocket tools. When true, makes the agent wait for a response, not call other tools and inform the user of the result. Only available for async custom_websocket tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_response: Option<bool>,
    /// The static context returned to the agent. Only present for custom_context tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

impl Tool {
    pub fn builder() -> ToolBuilder {
        <ToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolBuilder {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    project: Option<ToolProject>,
    r#type: Option<ToolType>,
    execution_mode: Option<ToolExecutionMode>,
    parameters: Option<Vec<ToolParameter>>,
    endpoint_method: Option<ToolEndpointMethod>,
    endpoint_url: Option<String>,
    endpoint_headers: Option<HashMap<String, String>>,
    endpoint_timeout_ms: Option<i64>,
    tool_call_output_timeout_ms: Option<i64>,
    phone_number: Option<String>,
    dtmf: Option<String>,
    dynamic_dtmf: Option<bool>,
    use_agent_phone_number: Option<bool>,
    detect_voicemail: Option<bool>,
    agents_to_transfer_to: Option<Vec<String>>,
    require_speech_before_tool_call: Option<bool>,
    wait_for_speech_before_tool_call: Option<bool>,
    forbid_speech_after_tool_call: Option<bool>,
    allow_tool_chaining: Option<bool>,
    wait_for_response: Option<bool>,
    context: Option<String>,
}

impl ToolBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn project(mut self, value: ToolProject) -> Self {
        self.project = Some(value);
        self
    }

    pub fn r#type(mut self, value: ToolType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn execution_mode(mut self, value: ToolExecutionMode) -> Self {
        self.execution_mode = Some(value);
        self
    }

    pub fn parameters(mut self, value: Vec<ToolParameter>) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn endpoint_method(mut self, value: ToolEndpointMethod) -> Self {
        self.endpoint_method = Some(value);
        self
    }

    pub fn endpoint_url(mut self, value: impl Into<String>) -> Self {
        self.endpoint_url = Some(value.into());
        self
    }

    pub fn endpoint_headers(mut self, value: HashMap<String, String>) -> Self {
        self.endpoint_headers = Some(value);
        self
    }

    pub fn endpoint_timeout_ms(mut self, value: i64) -> Self {
        self.endpoint_timeout_ms = Some(value);
        self
    }

    pub fn tool_call_output_timeout_ms(mut self, value: i64) -> Self {
        self.tool_call_output_timeout_ms = Some(value);
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn dtmf(mut self, value: impl Into<String>) -> Self {
        self.dtmf = Some(value.into());
        self
    }

    pub fn dynamic_dtmf(mut self, value: bool) -> Self {
        self.dynamic_dtmf = Some(value);
        self
    }

    pub fn use_agent_phone_number(mut self, value: bool) -> Self {
        self.use_agent_phone_number = Some(value);
        self
    }

    pub fn detect_voicemail(mut self, value: bool) -> Self {
        self.detect_voicemail = Some(value);
        self
    }

    pub fn agents_to_transfer_to(mut self, value: Vec<String>) -> Self {
        self.agents_to_transfer_to = Some(value);
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

    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Tool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ToolBuilder::id)
    /// - [`name`](ToolBuilder::name)
    /// - [`description`](ToolBuilder::description)
    /// - [`project`](ToolBuilder::project)
    /// - [`r#type`](ToolBuilder::r#type)
    /// - [`parameters`](ToolBuilder::parameters)
    pub fn build(self) -> Result<Tool, BuildError> {
        Ok(Tool {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            project: self.project.ok_or_else(|| BuildError::missing_field("project"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            execution_mode: self.execution_mode,
            parameters: self.parameters.ok_or_else(|| BuildError::missing_field("parameters"))?,
            endpoint_method: self.endpoint_method,
            endpoint_url: self.endpoint_url,
            endpoint_headers: self.endpoint_headers,
            endpoint_timeout_ms: self.endpoint_timeout_ms,
            tool_call_output_timeout_ms: self.tool_call_output_timeout_ms,
            phone_number: self.phone_number,
            dtmf: self.dtmf,
            dynamic_dtmf: self.dynamic_dtmf,
            use_agent_phone_number: self.use_agent_phone_number,
            detect_voicemail: self.detect_voicemail,
            agents_to_transfer_to: self.agents_to_transfer_to,
            require_speech_before_tool_call: self.require_speech_before_tool_call,
            wait_for_speech_before_tool_call: self.wait_for_speech_before_tool_call,
            forbid_speech_after_tool_call: self.forbid_speech_after_tool_call,
            allow_tool_chaining: self.allow_tool_chaining,
            wait_for_response: self.wait_for_response,
            context: self.context,
        })
    }
}
