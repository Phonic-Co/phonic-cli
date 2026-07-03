pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolCallPayload {
    pub r#type: String,
    /// Unique ID for this tool call
    #[serde(default)]
    pub tool_call_id: String,
    /// Name of the tool to execute
    #[serde(default)]
    pub tool_name: String,
    /// Parameters for tool execution
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
}

impl ToolCallPayload {
    pub fn builder() -> ToolCallPayloadBuilder {
        <ToolCallPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolCallPayloadBuilder {
    r#type: Option<String>,
    tool_call_id: Option<String>,
    tool_name: Option<String>,
    parameters: Option<HashMap<String, serde_json::Value>>,
}

impl ToolCallPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn tool_name(mut self, value: impl Into<String>) -> Self {
        self.tool_name = Some(value.into());
        self
    }

    pub fn parameters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolCallPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ToolCallPayloadBuilder::r#type)
    /// - [`tool_call_id`](ToolCallPayloadBuilder::tool_call_id)
    /// - [`tool_name`](ToolCallPayloadBuilder::tool_name)
    /// - [`parameters`](ToolCallPayloadBuilder::parameters)
    pub fn build(self) -> Result<ToolCallPayload, BuildError> {
        Ok(ToolCallPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
            parameters: self.parameters.ok_or_else(|| BuildError::missing_field("parameters"))?,
        })
    }
}
