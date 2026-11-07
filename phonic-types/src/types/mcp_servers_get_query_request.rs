pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpServersGetQueryRequest {
    /// The name of the project containing the MCP server. Only used when `nameOrId` is a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl McpServersGetQueryRequest {
    pub fn builder() -> McpServersGetQueryRequestBuilder {
        <McpServersGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServersGetQueryRequestBuilder {
    project: Option<String>,
}

impl McpServersGetQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpServersGetQueryRequest`].
    pub fn build(self) -> Result<McpServersGetQueryRequest, BuildError> {
        Ok(McpServersGetQueryRequest {
            project: self.project,
        })
    }
}

