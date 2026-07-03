pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationItemToolCallsItem {
    /// The tool call ID.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub tool: ConversationItemToolCallsItemTool,
    /// The integration associated with the tool, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration: Option<String>,
    /// HTTP method for webhook tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_method: Option<String>,
    /// URL for webhook tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// Headers for webhook tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_headers: Option<HashMap<String, Option<String>>>,
    /// Timeout in milliseconds for webhook tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_timeout_ms: Option<f64>,
    /// When the webhook endpoint was called (null on error).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_called_at: Option<DateTime<FixedOffset>>,
    /// Query parameters for webhook tool calls (null on error or when no params).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<HashMap<String, serde_json::Value>>,
    /// HTTP response status code for webhook tool calls (null on error).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status_code: Option<f64>,
    /// Timeout in milliseconds for websocket tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_output_timeout_ms: Option<f64>,
    /// The request body sent to the tool. Can be any JSON-serializable value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<ConversationItemToolCallsItemRequestBody>,
    /// The response body received from the tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_body: Option<HashMap<String, serde_json::Value>>,
    /// Whether the tool call timed out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_out: Option<bool>,
    /// Error message if the tool call failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl ConversationItemToolCallsItem {
    pub fn builder() -> ConversationItemToolCallsItemBuilder {
        <ConversationItemToolCallsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationItemToolCallsItemBuilder {
    id: Option<String>,
    tool: Option<ConversationItemToolCallsItemTool>,
    integration: Option<String>,
    endpoint_method: Option<String>,
    endpoint_url: Option<String>,
    endpoint_headers: Option<HashMap<String, Option<String>>>,
    endpoint_timeout_ms: Option<f64>,
    endpoint_called_at: Option<DateTime<FixedOffset>>,
    query_params: Option<HashMap<String, serde_json::Value>>,
    response_status_code: Option<f64>,
    tool_call_output_timeout_ms: Option<f64>,
    request_body: Option<ConversationItemToolCallsItemRequestBody>,
    response_body: Option<HashMap<String, serde_json::Value>>,
    timed_out: Option<bool>,
    error_message: Option<String>,
}

impl ConversationItemToolCallsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn tool(mut self, value: ConversationItemToolCallsItemTool) -> Self {
        self.tool = Some(value);
        self
    }

    pub fn integration(mut self, value: impl Into<String>) -> Self {
        self.integration = Some(value.into());
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

    pub fn endpoint_headers(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.endpoint_headers = Some(value);
        self
    }

    pub fn endpoint_timeout_ms(mut self, value: f64) -> Self {
        self.endpoint_timeout_ms = Some(value);
        self
    }

    pub fn endpoint_called_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.endpoint_called_at = Some(value);
        self
    }

    pub fn query_params(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.query_params = Some(value);
        self
    }

    pub fn response_status_code(mut self, value: f64) -> Self {
        self.response_status_code = Some(value);
        self
    }

    pub fn tool_call_output_timeout_ms(mut self, value: f64) -> Self {
        self.tool_call_output_timeout_ms = Some(value);
        self
    }

    pub fn request_body(mut self, value: ConversationItemToolCallsItemRequestBody) -> Self {
        self.request_body = Some(value);
        self
    }

    pub fn response_body(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.response_body = Some(value);
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

    /// Consumes the builder and constructs a [`ConversationItemToolCallsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationItemToolCallsItemBuilder::id)
    /// - [`tool`](ConversationItemToolCallsItemBuilder::tool)
    pub fn build(self) -> Result<ConversationItemToolCallsItem, BuildError> {
        Ok(ConversationItemToolCallsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            tool: self.tool.ok_or_else(|| BuildError::missing_field("tool"))?,
            integration: self.integration,
            endpoint_method: self.endpoint_method,
            endpoint_url: self.endpoint_url,
            endpoint_headers: self.endpoint_headers,
            endpoint_timeout_ms: self.endpoint_timeout_ms,
            endpoint_called_at: self.endpoint_called_at,
            query_params: self.query_params,
            response_status_code: self.response_status_code,
            tool_call_output_timeout_ms: self.tool_call_output_timeout_ms,
            request_body: self.request_body,
            response_body: self.response_body,
            timed_out: self.timed_out,
            error_message: self.error_message,
        })
    }
}
