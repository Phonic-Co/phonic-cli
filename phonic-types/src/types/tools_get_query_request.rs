pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolsGetQueryRequest {
    /// The name of the project containing the tool. Only used when `nameOrId` is a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ToolsGetQueryRequest {
    pub fn builder() -> ToolsGetQueryRequestBuilder {
        <ToolsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsGetQueryRequestBuilder {
    project: Option<String>,
}

impl ToolsGetQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolsGetQueryRequest`].
    pub fn build(self) -> Result<ToolsGetQueryRequest, BuildError> {
        Ok(ToolsGetQueryRequest {
            project: self.project,
        })
    }
}

