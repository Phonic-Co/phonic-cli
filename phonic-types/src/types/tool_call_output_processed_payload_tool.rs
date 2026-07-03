pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolCallOutputProcessedPayloadTool {
    /// Tool ID
    #[serde(default)]
    pub id: String,
    /// Human-readable tool name
    #[serde(default)]
    pub name: String,
}

impl ToolCallOutputProcessedPayloadTool {
    pub fn builder() -> ToolCallOutputProcessedPayloadToolBuilder {
        <ToolCallOutputProcessedPayloadToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolCallOutputProcessedPayloadToolBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ToolCallOutputProcessedPayloadToolBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolCallOutputProcessedPayloadTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ToolCallOutputProcessedPayloadToolBuilder::id)
    /// - [`name`](ToolCallOutputProcessedPayloadToolBuilder::name)
    pub fn build(self) -> Result<ToolCallOutputProcessedPayloadTool, BuildError> {
        Ok(ToolCallOutputProcessedPayloadTool {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
