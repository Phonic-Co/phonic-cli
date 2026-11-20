pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for disable
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisableQueryRequest {
    /// The name of the project containing the agent. Only used when `nameOrId` is a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl DisableQueryRequest {
    pub fn builder() -> DisableQueryRequestBuilder {
        <DisableQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisableQueryRequestBuilder {
    project: Option<String>,
}

impl DisableQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisableQueryRequest`].
    pub fn build(self) -> Result<DisableQueryRequest, BuildError> {
        Ok(DisableQueryRequest {
            project: self.project,
        })
    }
}

