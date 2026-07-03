pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolsGetResponse {
    pub tool: Tool,
}

impl ToolsGetResponse {
    pub fn builder() -> ToolsGetResponseBuilder {
        <ToolsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsGetResponseBuilder {
    tool: Option<Tool>,
}

impl ToolsGetResponseBuilder {
    pub fn tool(mut self, value: Tool) -> Self {
        self.tool = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tool`](ToolsGetResponseBuilder::tool)
    pub fn build(self) -> Result<ToolsGetResponse, BuildError> {
        Ok(ToolsGetResponse {
            tool: self.tool.ok_or_else(|| BuildError::missing_field("tool"))?,
        })
    }
}
