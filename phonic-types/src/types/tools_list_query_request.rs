pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolsListQueryRequest {
    /// The name of the project to list tools for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ToolsListQueryRequest {
    pub fn builder() -> ToolsListQueryRequestBuilder {
        <ToolsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsListQueryRequestBuilder {
    project: Option<String>,
}

impl ToolsListQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolsListQueryRequest`].
    pub fn build(self) -> Result<ToolsListQueryRequest, BuildError> {
        Ok(ToolsListQueryRequest {
            project: self.project,
        })
    }
}

