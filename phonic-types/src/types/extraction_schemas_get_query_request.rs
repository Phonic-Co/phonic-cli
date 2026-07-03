pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExtractionSchemasGetQueryRequest {
    /// The name of the project containing the extraction schema. Only used when `nameOrId` is a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ExtractionSchemasGetQueryRequest {
    pub fn builder() -> ExtractionSchemasGetQueryRequestBuilder {
        <ExtractionSchemasGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractionSchemasGetQueryRequestBuilder {
    project: Option<String>,
}

impl ExtractionSchemasGetQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExtractionSchemasGetQueryRequest`].
    pub fn build(self) -> Result<ExtractionSchemasGetQueryRequest, BuildError> {
        Ok(ExtractionSchemasGetQueryRequest {
            project: self.project,
        })
    }
}

