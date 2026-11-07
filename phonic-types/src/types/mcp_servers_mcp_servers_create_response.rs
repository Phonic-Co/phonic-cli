pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpServersCreateResponse {
    /// The ID of the created MCP server.
    #[serde(default)]
    pub id: String,
    /// The name of the created MCP server.
    #[serde(default)]
    pub name: String,
}

impl McpServersCreateResponse {
    pub fn builder() -> McpServersCreateResponseBuilder {
        <McpServersCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServersCreateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl McpServersCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpServersCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](McpServersCreateResponseBuilder::id)
    /// - [`name`](McpServersCreateResponseBuilder::name)
    pub fn build(self) -> Result<McpServersCreateResponse, BuildError> {
        Ok(McpServersCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
