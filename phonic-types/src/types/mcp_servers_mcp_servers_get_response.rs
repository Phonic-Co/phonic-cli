pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct McpServersGetResponse {
    #[serde(default)]
    pub mcp_server: McpServer,
}

impl McpServersGetResponse {
    pub fn builder() -> McpServersGetResponseBuilder {
        <McpServersGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServersGetResponseBuilder {
    mcp_server: Option<McpServer>,
}

impl McpServersGetResponseBuilder {
    pub fn mcp_server(mut self, value: McpServer) -> Self {
        self.mcp_server = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServersGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mcp_server`](McpServersGetResponseBuilder::mcp_server)
    pub fn build(self) -> Result<McpServersGetResponse, BuildError> {
        Ok(McpServersGetResponse {
            mcp_server: self.mcp_server.ok_or_else(|| BuildError::missing_field("mcp_server"))?,
        })
    }
}
