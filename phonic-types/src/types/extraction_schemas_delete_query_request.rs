pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExtractionSchemasDeleteQueryRequest {
    /// The name of the project containing the extraction schema. Only used when `nameOrId` is a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ExtractionSchemasDeleteQueryRequest {
    pub fn builder() -> ExtractionSchemasDeleteQueryRequestBuilder {
        <ExtractionSchemasDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractionSchemasDeleteQueryRequestBuilder {
    project: Option<String>,
}

impl ExtractionSchemasDeleteQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExtractionSchemasDeleteQueryRequest`].
    pub fn build(self) -> Result<ExtractionSchemasDeleteQueryRequest, BuildError> {
        Ok(ExtractionSchemasDeleteQueryRequest {
            project: self.project,
        })
    }
}

