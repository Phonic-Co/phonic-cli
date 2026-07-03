pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExtractionSchemasListQueryRequest {
    /// The name of the project to list extraction schemas for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ExtractionSchemasListQueryRequest {
    pub fn builder() -> ExtractionSchemasListQueryRequestBuilder {
        <ExtractionSchemasListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractionSchemasListQueryRequestBuilder {
    project: Option<String>,
}

impl ExtractionSchemasListQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExtractionSchemasListQueryRequest`].
    pub fn build(self) -> Result<ExtractionSchemasListQueryRequest, BuildError> {
        Ok(ExtractionSchemasListQueryRequest {
            project: self.project,
        })
    }
}

