pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A tool call the assistant made earlier in the conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesToolCall {
    /// Identifier for this tool call, unique within `input`.
    #[serde(default)]
    pub tool_call_id: String,
    /// Name of the called tool.
    #[serde(default)]
    pub tool_name: String,
    /// Arguments the assistant passed in the request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<HashMap<String, serde_json::Value>>,
    /// Arguments the assistant passed as query parameters. An argument cannot appear in both `request_body` and `query_params`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<HashMap<String, Option<String>>>,
}

impl ResponsesToolCall {
    pub fn builder() -> ResponsesToolCallBuilder {
        <ResponsesToolCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesToolCallBuilder {
    tool_call_id: Option<String>,
    tool_name: Option<String>,
    request_body: Option<HashMap<String, serde_json::Value>>,
    query_params: Option<HashMap<String, Option<String>>>,
}

impl ResponsesToolCallBuilder {
    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn request_body(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.request_body = Some(value);
        self
    }

    pub fn query_params(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.query_params = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesToolCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_call_id`](ResponsesToolCallBuilder::tool_call_id)
    /// - [`tool_name`](ResponsesToolCallBuilder::tool_name)
    pub fn build(self) -> Result<ResponsesToolCall, BuildError> {
        Ok(ResponsesToolCall {
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
            request_body: self.request_body,
            query_params: self.query_params,
        })
    }
}
