pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ToolCallInterruptedPayload {
    pub r#type: String,
    /// ID of the interrupted tool call
    #[serde(default)]
    pub tool_call_id: String,
    /// Name of the interrupted tool
    #[serde(default)]
    pub tool_name: String,
}

impl ToolCallInterruptedPayload {
    pub fn builder() -> ToolCallInterruptedPayloadBuilder {
        <ToolCallInterruptedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolCallInterruptedPayloadBuilder {
    r#type: Option<String>,
    tool_call_id: Option<String>,
    tool_name: Option<String>,
}

impl ToolCallInterruptedPayloadBuilder {
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

    /// Consumes the builder and constructs a [`ToolCallInterruptedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ToolCallInterruptedPayloadBuilder::r#type)
    /// - [`tool_call_id`](ToolCallInterruptedPayloadBuilder::tool_call_id)
    /// - [`tool_name`](ToolCallInterruptedPayloadBuilder::tool_name)
    pub fn build(self) -> Result<ToolCallInterruptedPayload, BuildError> {
        Ok(ToolCallInterruptedPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_name: self.tool_name.ok_or_else(|| BuildError::missing_field("tool_name"))?,
        })
    }
}
