pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCallOutputProcessedPayload {
    pub r#type: String,
    /// ID of the completed tool call
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub tool: ToolCallOutputProcessedPayloadTool,
    /// Configuration of the tool that was called
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_config: Option<HashMap<String, serde_json::Value>>,
    /// HTTP method used for webhook endpoint (null for WebSocket tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_method: Option<String>,
    /// Webhook endpoint URL (null for WebSocket tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// Webhook timeout in milliseconds (null for WebSocket tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_timeout_ms: Option<i64>,
    /// When webhook was called (null for WebSocket tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_called_at: Option<DateTime<FixedOffset>>,
    /// Query string parameters sent to webhook endpoint (null for WebSocket tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<HashMap<String, Option<String>>>,
    /// Webhook request body (null for WebSocket tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<HashMap<String, serde_json::Value>>,
    /// Webhook response body (null for WebSocket tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_body: Option<serde_json::Value>,
    /// WebSocket tool parameters (null for webhook tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, serde_json::Value>>,
    /// WebSocket tool output (null for webhook tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<serde_json::Value>,
    /// Webhook HTTP status code (null for WebSocket tools)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status_code: Option<i64>,
    /// Duration of the tool call in milliseconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<f64>,
    /// Whether the tool call timed out
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_out: Option<bool>,
    /// Error message if tool call failed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ToolCallOutputProcessedPayload {
    pub fn builder() -> ToolCallOutputProcessedPayloadBuilder {
        <ToolCallOutputProcessedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolCallOutputProcessedPayloadBuilder {
    r#type: Option<String>,
    tool_call_id: Option<String>,
    tool: Option<ToolCallOutputProcessedPayloadTool>,
    tool_config: Option<HashMap<String, serde_json::Value>>,
    endpoint_method: Option<String>,
    endpoint_url: Option<String>,
    endpoint_timeout_ms: Option<i64>,
    endpoint_called_at: Option<DateTime<FixedOffset>>,
    query_params: Option<HashMap<String, Option<String>>>,
    request_body: Option<HashMap<String, serde_json::Value>>,
    response_body: Option<serde_json::Value>,
    parameters: Option<HashMap<String, serde_json::Value>>,
    output: Option<serde_json::Value>,
    response_status_code: Option<i64>,
    duration_ms: Option<f64>,
    timed_out: Option<bool>,
    error_message: Option<String>,
}

impl ToolCallOutputProcessedPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn tool(mut self, value: ToolCallOutputProcessedPayloadTool) -> Self {
        self.tool = Some(value);
        self
    }

    pub fn tool_config(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.tool_config = Some(value);
        self
    }

    pub fn endpoint_method(mut self, value: impl Into<String>) -> Self {
        self.endpoint_method = Some(value.into());
        self
    }

    pub fn endpoint_url(mut self, value: impl Into<String>) -> Self {
        self.endpoint_url = Some(value.into());
        self
    }

    pub fn endpoint_timeout_ms(mut self, value: i64) -> Self {
        self.endpoint_timeout_ms = Some(value);
        self
    }

    pub fn endpoint_called_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.endpoint_called_at = Some(value);
        self
    }

    pub fn query_params(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.query_params = Some(value);
        self
    }

    pub fn request_body(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.request_body = Some(value);
        self
    }

    pub fn response_body(mut self, value: serde_json::Value) -> Self {
        self.response_body = Some(value);
        self
    }

    pub fn parameters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn output(mut self, value: serde_json::Value) -> Self {
        self.output = Some(value);
        self
    }

    pub fn response_status_code(mut self, value: i64) -> Self {
        self.response_status_code = Some(value);
        self
    }

    pub fn duration_ms(mut self, value: f64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn timed_out(mut self, value: bool) -> Self {
        self.timed_out = Some(value);
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolCallOutputProcessedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ToolCallOutputProcessedPayloadBuilder::r#type)
    /// - [`tool_call_id`](ToolCallOutputProcessedPayloadBuilder::tool_call_id)
    /// - [`tool`](ToolCallOutputProcessedPayloadBuilder::tool)
    pub fn build(self) -> Result<ToolCallOutputProcessedPayload, BuildError> {
        Ok(ToolCallOutputProcessedPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool: self.tool.ok_or_else(|| BuildError::missing_field("tool"))?,
            tool_config: self.tool_config,
            endpoint_method: self.endpoint_method,
            endpoint_url: self.endpoint_url,
            endpoint_timeout_ms: self.endpoint_timeout_ms,
            endpoint_called_at: self.endpoint_called_at,
            query_params: self.query_params,
            request_body: self.request_body,
            response_body: self.response_body,
            parameters: self.parameters,
            output: self.output,
            response_status_code: self.response_status_code,
            duration_ms: self.duration_ms,
            timed_out: self.timed_out,
            error_message: self.error_message,
        })
    }
}
