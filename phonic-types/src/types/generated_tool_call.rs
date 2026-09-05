pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeneratedToolCall {
    /// Identifier for this tool call. Send it back as the `tool_call_id` of the matching `tool_call_output` input item.
    #[serde(default)]
    pub tool_call_id: String,
    /// Name of the tool to call. Always one of the `tool_definitions` from the request.
    #[serde(default)]
    pub tool_name: String,
    /// Arguments the assistant produced for the request body.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<HashMap<String, serde_json::Value>>,
    /// Arguments the assistant produced as query parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_params: Option<HashMap<String, String>>,
}

impl GeneratedToolCall {
    pub fn builder() -> GeneratedToolCallBuilder {
        <GeneratedToolCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneratedToolCallBuilder {
    tool_call_id: Option<String>,
    tool_name: Option<String>,
    request_body: Option<HashMap<String, serde_json::Value>>,
    query_params: Option<HashMap<String, String>>,
}

impl GeneratedToolCallBuilder {
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

    pub fn query_params(mut self, value: HashMap<String, String>) -> Self {
        self.query_params = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeneratedToolCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool_call_id`](GeneratedToolCallBuilder::tool_call_id)
    /// - [`tool_name`](GeneratedToolCallBuilder::tool_name)
    pub fn build(self) -> Result<GeneratedToolCall, BuildError> {
        Ok(GeneratedToolCall {
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
            request_body: self.request_body,
            query_params: self.query_params,
        })
    }
}
