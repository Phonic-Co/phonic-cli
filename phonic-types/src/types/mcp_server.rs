pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A customer-hosted MCP server whose tools can be made available to agents.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpServer {
    /// The ID of the MCP server.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub project: McpServerProject,
    /// The name of the MCP server.
    #[serde(default)]
    pub name: String,
    /// The URL of the MCP server.
    #[serde(default)]
    pub url: String,
    /// Names of the headers sent to the MCP server. Header values are write-only and never returned.
    #[serde(default)]
    pub header_names: Vec<String>,
    /// Tools listed by the MCP server when it was created.
    #[serde(default)]
    pub tools: Vec<McpServerTool>,
}

impl McpServer {
    pub fn builder() -> McpServerBuilder {
        <McpServerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerBuilder {
    id: Option<String>,
    project: Option<McpServerProject>,
    name: Option<String>,
    url: Option<String>,
    header_names: Option<Vec<String>>,
    tools: Option<Vec<McpServerTool>>,
}

impl McpServerBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn project(mut self, value: McpServerProject) -> Self {
        self.project = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn header_names(mut self, value: Vec<String>) -> Self {
        self.header_names = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<McpServerTool>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](McpServerBuilder::id)
    /// - [`project`](McpServerBuilder::project)
    /// - [`name`](McpServerBuilder::name)
    /// - [`url`](McpServerBuilder::url)
    /// - [`header_names`](McpServerBuilder::header_names)
    /// - [`tools`](McpServerBuilder::tools)
    pub fn build(self) -> Result<McpServer, BuildError> {
        Ok(McpServer {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            project: self.project.ok_or_else(|| BuildError::missing_field("project"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            header_names: self.header_names.ok_or_else(|| BuildError::missing_field("header_names"))?,
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
        })
    }
}
