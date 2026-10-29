pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for enable
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnableQueryRequest {
    /// The name of the project containing the agent. Only used when `nameOrId` is a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl EnableQueryRequest {
    pub fn builder() -> EnableQueryRequestBuilder {
        <EnableQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnableQueryRequestBuilder {
    project: Option<String>,
}

impl EnableQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EnableQueryRequest`].
    pub fn build(self) -> Result<EnableQueryRequest, BuildError> {
        Ok(EnableQueryRequest {
            project: self.project,
        })
    }
}

