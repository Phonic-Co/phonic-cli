pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReplayToolCall {
    #[serde(default)]
    pub tool: ReplayToolCallTool,
    /// HTTP method for webhook tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_method: Option<ReplayToolCallEndpointMethod>,
    /// URL for webhook tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// Headers for webhook tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_headers: Option<HashMap<String, Option<String>>>,
    /// Query parameters the LLM produced for webhook tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<HashMap<String, Option<String>>>,
    /// Context returned from custom-context tool calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    /// Request body the LLM produced for the tool call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<HashMap<String, serde_json::Value>>,
}

impl ReplayToolCall {
    pub fn builder() -> ReplayToolCallBuilder {
        <ReplayToolCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayToolCallBuilder {
    tool: Option<ReplayToolCallTool>,
    endpoint_method: Option<ReplayToolCallEndpointMethod>,
    endpoint_url: Option<String>,
    endpoint_headers: Option<HashMap<String, Option<String>>>,
    query_params: Option<HashMap<String, Option<String>>>,
    context: Option<String>,
    request_body: Option<HashMap<String, serde_json::Value>>,
}

impl ReplayToolCallBuilder {
    pub fn tool(mut self, value: ReplayToolCallTool) -> Self {
        self.tool = Some(value);
        self
    }

    pub fn endpoint_method(mut self, value: ReplayToolCallEndpointMethod) -> Self {
        self.endpoint_method = Some(value);
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

    pub fn query_params(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.query_params = Some(value);
        self
    }

    pub fn context(mut self, value: impl Into<String>) -> Self {
        self.context = Some(value.into());
        self
    }

    pub fn request_body(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.request_body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplayToolCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool`](ReplayToolCallBuilder::tool)
    pub fn build(self) -> Result<ReplayToolCall, BuildError> {
        Ok(ReplayToolCall {
            tool: self.tool.ok_or_else(|| BuildError::missing_field("tool"))?,
            endpoint_method: self.endpoint_method,
            endpoint_url: self.endpoint_url,
            endpoint_headers: self.endpoint_headers,
            query_params: self.query_params,
            context: self.context,
            request_body: self.request_body,
        })
    }
}
