pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExtractionSchemasCreateResponse {
    /// The ID of the created extraction schema.
    #[serde(default)]
    pub id: String,
    /// The name of the created extraction schema.
    #[serde(default)]
    pub name: String,
}

impl ExtractionSchemasCreateResponse {
    pub fn builder() -> ExtractionSchemasCreateResponseBuilder {
        <ExtractionSchemasCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractionSchemasCreateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ExtractionSchemasCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExtractionSchemasCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExtractionSchemasCreateResponseBuilder::id)
    /// - [`name`](ExtractionSchemasCreateResponseBuilder::name)
    pub fn build(self) -> Result<ExtractionSchemasCreateResponse, BuildError> {
        Ok(ExtractionSchemasCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
