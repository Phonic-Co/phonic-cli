pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Request for create (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateRequest {
    /// The name of the project to create the agent in.
    #[serde(skip)]
    pub project: Option<String>,
    #[serde(default)]
    pub body: CreateAgentRequest,
}

impl CreateRequest {
    pub fn builder() -> CreateRequestBuilder {
        <CreateRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateRequestBuilder {
    project: Option<String>,
    body: Option<CreateAgentRequest>,
}

impl CreateRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    pub fn body(mut self, value: CreateAgentRequest) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](CreateRequestBuilder::body)
    pub fn build(self) -> Result<CreateRequest, BuildError> {
        Ok(CreateRequest {
            project: self.project,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}

