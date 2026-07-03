pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsGetQueryRequest {
    /// The name of the project containing the agent. Only used when `nameOrId` is a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl AgentsGetQueryRequest {
    pub fn builder() -> AgentsGetQueryRequestBuilder {
        <AgentsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsGetQueryRequestBuilder {
    project: Option<String>,
}

impl AgentsGetQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsGetQueryRequest`].
    pub fn build(self) -> Result<AgentsGetQueryRequest, BuildError> {
        Ok(AgentsGetQueryRequest {
            project: self.project,
        })
    }
}

