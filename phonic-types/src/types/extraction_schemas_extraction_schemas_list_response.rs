pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExtractionSchemasListResponse {
    #[serde(default)]
    pub extraction_schemas: Vec<ExtractionSchema>,
}

impl ExtractionSchemasListResponse {
    pub fn builder() -> ExtractionSchemasListResponseBuilder {
        <ExtractionSchemasListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractionSchemasListResponseBuilder {
    extraction_schemas: Option<Vec<ExtractionSchema>>,
}

impl ExtractionSchemasListResponseBuilder {
    pub fn extraction_schemas(mut self, value: Vec<ExtractionSchema>) -> Self {
        self.extraction_schemas = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExtractionSchemasListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`extraction_schemas`](ExtractionSchemasListResponseBuilder::extraction_schemas)
    pub fn build(self) -> Result<ExtractionSchemasListResponse, BuildError> {
        Ok(ExtractionSchemasListResponse {
            extraction_schemas: self.extraction_schemas.ok_or_else(|| BuildError::missing_field("extraction_schemas"))?,
        })
    }
}
