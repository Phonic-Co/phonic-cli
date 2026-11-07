pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpServersDeleteQueryRequest {
    /// The name of the project containing the MCP server. Only used when `nameOrId` is a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl McpServersDeleteQueryRequest {
    pub fn builder() -> McpServersDeleteQueryRequestBuilder {
        <McpServersDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServersDeleteQueryRequestBuilder {
    project: Option<String>,
}

impl McpServersDeleteQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`McpServersDeleteQueryRequest`].
    pub fn build(self) -> Result<McpServersDeleteQueryRequest, BuildError> {
        Ok(McpServersDeleteQueryRequest {
            project: self.project,
        })
    }
}

