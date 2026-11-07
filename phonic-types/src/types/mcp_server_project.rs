pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpServerProject {
    /// The ID of the project containing the MCP server.
    #[serde(default)]
    pub id: String,
    /// The name of the project containing the MCP server.
    #[serde(default)]
    pub name: String,
}

impl McpServerProject {
    pub fn builder() -> McpServerProjectBuilder {
        <McpServerProjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerProjectBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl McpServerProjectBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpServerProject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](McpServerProjectBuilder::id)
    /// - [`name`](McpServerProjectBuilder::name)
    pub fn build(self) -> Result<McpServerProject, BuildError> {
        Ok(McpServerProject {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
