pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExtractionSchemasGetResponse {
    #[serde(default)]
    pub extraction_schema: ExtractionSchema,
}

impl ExtractionSchemasGetResponse {
    pub fn builder() -> ExtractionSchemasGetResponseBuilder {
        <ExtractionSchemasGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractionSchemasGetResponseBuilder {
    extraction_schema: Option<ExtractionSchema>,
}

impl ExtractionSchemasGetResponseBuilder {
    pub fn extraction_schema(mut self, value: ExtractionSchema) -> Self {
        self.extraction_schema = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExtractionSchemasGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`extraction_schema`](ExtractionSchemasGetResponseBuilder::extraction_schema)
    pub fn build(self) -> Result<ExtractionSchemasGetResponse, BuildError> {
        Ok(ExtractionSchemasGetResponse {
            extraction_schema: self.extraction_schema.ok_or_else(|| BuildError::missing_field("extraction_schema"))?,
        })
    }
}
