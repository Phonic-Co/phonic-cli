pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpServersDeleteResponse {
    /// Whether the MCP server was deleted successfully.
    #[serde(default)]
    pub success: bool,
}

impl McpServersDeleteResponse {
    pub fn builder() -> McpServersDeleteResponseBuilder {
        <McpServersDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServersDeleteResponseBuilder {
    success: Option<bool>,
}

impl McpServersDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServersDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](McpServersDeleteResponseBuilder::success)
    pub fn build(self) -> Result<McpServersDeleteResponse, BuildError> {
        Ok(McpServersDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
