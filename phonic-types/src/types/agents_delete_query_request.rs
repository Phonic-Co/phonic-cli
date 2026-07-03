pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsDeleteQueryRequest {
    /// The name of the project containing the agent. Only used when `nameOrId` is a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl AgentsDeleteQueryRequest {
    pub fn builder() -> AgentsDeleteQueryRequestBuilder {
        <AgentsDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsDeleteQueryRequestBuilder {
    project: Option<String>,
}

impl AgentsDeleteQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsDeleteQueryRequest`].
    pub fn build(self) -> Result<AgentsDeleteQueryRequest, BuildError> {
        Ok(AgentsDeleteQueryRequest {
            project: self.project,
        })
    }
}

