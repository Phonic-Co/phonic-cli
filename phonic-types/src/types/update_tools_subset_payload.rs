pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateToolsSubsetPayload {
    pub r#type: String,
    /// Tools available to the assistant. Use a string to reference a pre-defined tool by name, or define an inline WebSocket tool for this conversation. Tool names must be unique.
    #[serde(default)]
    pub tools: Vec<ToolDefinition>,
}

impl UpdateToolsSubsetPayload {
    pub fn builder() -> UpdateToolsSubsetPayloadBuilder {
        <UpdateToolsSubsetPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateToolsSubsetPayloadBuilder {
    r#type: Option<String>,
    tools: Option<Vec<ToolDefinition>>,
}

impl UpdateToolsSubsetPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn tools(mut self, value: Vec<ToolDefinition>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateToolsSubsetPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](UpdateToolsSubsetPayloadBuilder::r#type)
    /// - [`tools`](UpdateToolsSubsetPayloadBuilder::tools)
    pub fn build(self) -> Result<UpdateToolsSubsetPayload, BuildError> {
        Ok(UpdateToolsSubsetPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
        })
    }
}
