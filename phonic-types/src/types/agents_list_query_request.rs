pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsListQueryRequest {
    /// The name of the project to list agents for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl AgentsListQueryRequest {
    pub fn builder() -> AgentsListQueryRequestBuilder {
        <AgentsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsListQueryRequestBuilder {
    project: Option<String>,
}

impl AgentsListQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsListQueryRequest`].
    pub fn build(self) -> Result<AgentsListQueryRequest, BuildError> {
        Ok(AgentsListQueryRequest {
            project: self.project,
        })
    }
}

