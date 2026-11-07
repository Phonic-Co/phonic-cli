pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpServersListResponse {
    #[serde(default)]
    pub mcp_servers: Vec<McpServer>,
}

impl McpServersListResponse {
    pub fn builder() -> McpServersListResponseBuilder {
        <McpServersListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServersListResponseBuilder {
    mcp_servers: Option<Vec<McpServer>>,
}

impl McpServersListResponseBuilder {
    pub fn mcp_servers(mut self, value: Vec<McpServer>) -> Self {
        self.mcp_servers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServersListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mcp_servers`](McpServersListResponseBuilder::mcp_servers)
    pub fn build(self) -> Result<McpServersListResponse, BuildError> {
        Ok(McpServersListResponse {
            mcp_servers: self.mcp_servers.ok_or_else(|| BuildError::missing_field("mcp_servers"))?,
        })
    }
}
