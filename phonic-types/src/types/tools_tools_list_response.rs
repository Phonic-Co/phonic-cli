pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ToolsListResponse {
    #[serde(default)]
    pub tools: Vec<Tool>,
}

impl ToolsListResponse {
    pub fn builder() -> ToolsListResponseBuilder {
        <ToolsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsListResponseBuilder {
    tools: Option<Vec<Tool>>,
}

impl ToolsListResponseBuilder {
    pub fn tools(mut self, value: Vec<Tool>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tools`](ToolsListResponseBuilder::tools)
    pub fn build(self) -> Result<ToolsListResponse, BuildError> {
        Ok(ToolsListResponse {
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
        })
    }
}
