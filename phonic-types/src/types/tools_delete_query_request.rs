pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolsDeleteQueryRequest {
    /// The name of the project containing the tool. Only used when `nameOrId` is a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ToolsDeleteQueryRequest {
    pub fn builder() -> ToolsDeleteQueryRequestBuilder {
        <ToolsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsDeleteQueryRequestBuilder {
    project: Option<String>,
}

impl ToolsDeleteQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolsDeleteQueryRequest`].
    pub fn build(self) -> Result<ToolsDeleteQueryRequest, BuildError> {
        Ok(ToolsDeleteQueryRequest {
            project: self.project,
        })
    }
}

