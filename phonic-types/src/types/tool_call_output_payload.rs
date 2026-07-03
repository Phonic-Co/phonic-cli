pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCallOutputPayload {
    pub r#type: String,
    /// ID of the tool call being responded to
    #[serde(default)]
    pub tool_call_id: String,
    pub output: serde_json::Value,
}

impl ToolCallOutputPayload {
    pub fn builder() -> ToolCallOutputPayloadBuilder {
        <ToolCallOutputPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolCallOutputPayloadBuilder {
    r#type: Option<String>,
    tool_call_id: Option<String>,
    output: Option<serde_json::Value>,
}

impl ToolCallOutputPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn output(mut self, value: serde_json::Value) -> Self {
        self.output = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolCallOutputPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ToolCallOutputPayloadBuilder::r#type)
    /// - [`tool_call_id`](ToolCallOutputPayloadBuilder::tool_call_id)
    /// - [`output`](ToolCallOutputPayloadBuilder::output)
    pub fn build(self) -> Result<ToolCallOutputPayload, BuildError> {
        Ok(ToolCallOutputPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            output: self.output.ok_or_else(|| BuildError::missing_field("output"))?,
        })
    }
}
