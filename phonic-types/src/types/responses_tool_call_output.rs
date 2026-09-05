pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The result of an assistant tool call. Must record an outcome through `response_body`, `response_status_code`, `error_message`, `timed_out`, or `interrupted`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesToolCallOutput {
    pub r#type: String,
    /// The `tool_call_id` of the assistant tool call this output belongs to.
    #[serde(default)]
    pub tool_call_id: String,
    /// The value the tool returned. Can be any JSON-serializable value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_body: Option<ResponsesToolCallOutputResponseBody>,
    /// HTTP status code the tool returned, for tools called over HTTP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status_code: Option<i64>,
    /// Whether the tool call timed out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_out: Option<bool>,
    /// The error the tool call failed with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Whether the tool call was interrupted before it finished.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interrupted: Option<bool>,
}

impl ResponsesToolCallOutput {
    pub fn builder() -> ResponsesToolCallOutputBuilder {
        <ResponsesToolCallOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesToolCallOutputBuilder {
    r#type: Option<String>,
    tool_call_id: Option<String>,
    response_body: Option<ResponsesToolCallOutputResponseBody>,
    response_status_code: Option<i64>,
    timed_out: Option<bool>,
    error_message: Option<String>,
    interrupted: Option<bool>,
}

impl ResponsesToolCallOutputBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn response_body(mut self, value: ResponsesToolCallOutputResponseBody) -> Self {
        self.response_body = Some(value);
        self
    }

    pub fn response_status_code(mut self, value: i64) -> Self {
        self.response_status_code = Some(value);
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

    pub fn interrupted(mut self, value: bool) -> Self {
        self.interrupted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesToolCallOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ResponsesToolCallOutputBuilder::r#type)
    /// - [`tool_call_id`](ResponsesToolCallOutputBuilder::tool_call_id)
    pub fn build(self) -> Result<ResponsesToolCallOutput, BuildError> {
        Ok(ResponsesToolCallOutput {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            response_body: self.response_body,
            response_status_code: self.response_status_code,
            timed_out: self.timed_out,
            error_message: self.error_message,
            interrupted: self.interrupted,
        })
    }
}
