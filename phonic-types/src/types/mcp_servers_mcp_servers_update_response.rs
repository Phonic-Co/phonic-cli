pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct McpServersUpdateResponse {
    /// Whether the MCP server was updated successfully.
    #[serde(default)]
    pub success: bool,
}

impl McpServersUpdateResponse {
    pub fn builder() -> McpServersUpdateResponseBuilder {
        <McpServersUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServersUpdateResponseBuilder {
    success: Option<bool>,
}

impl McpServersUpdateResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServersUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](McpServersUpdateResponseBuilder::success)
    pub fn build(self) -> Result<McpServersUpdateResponse, BuildError> {
        Ok(McpServersUpdateResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
