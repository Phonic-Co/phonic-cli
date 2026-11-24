pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A tool listed by an MCP server.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpServerTool {
    /// The name of the tool on the MCP server.
    #[serde(default)]
    pub name: String,
    /// The description of the tool.
    #[serde(default)]
    pub description: String,
    /// JSON Schema describing the tool's arguments.
    pub parameters: McpServerToolParameters,
}

impl McpServerTool {
    pub fn builder() -> McpServerToolBuilder {
        <McpServerToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerToolBuilder {
    name: Option<String>,
    description: Option<String>,
    parameters: Option<McpServerToolParameters>,
}

impl McpServerToolBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn parameters(mut self, value: McpServerToolParameters) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServerTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](McpServerToolBuilder::name)
    /// - [`description`](McpServerToolBuilder::description)
    /// - [`parameters`](McpServerToolBuilder::parameters)
    pub fn build(self) -> Result<McpServerTool, BuildError> {
        Ok(McpServerTool {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            parameters: self.parameters.ok_or_else(|| BuildError::missing_field("parameters"))?,
        })
    }
}
