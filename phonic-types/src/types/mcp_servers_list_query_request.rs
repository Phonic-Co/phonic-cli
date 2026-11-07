pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpServersListQueryRequest {
    /// The name of the project to list MCP servers for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl McpServersListQueryRequest {
    pub fn builder() -> McpServersListQueryRequestBuilder {
        <McpServersListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServersListQueryRequestBuilder {
    project: Option<String>,
}

impl McpServersListQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpServersListQueryRequest`].
    pub fn build(self) -> Result<McpServersListQueryRequest, BuildError> {
        Ok(McpServersListQueryRequest {
            project: self.project,
        })
    }
}

