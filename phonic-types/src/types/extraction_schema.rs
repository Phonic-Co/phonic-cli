pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExtractionSchema {
    /// The ID of the extraction schema.
    #[serde(default)]
    pub id: String,
    /// The name of the extraction schema.
    #[serde(default)]
    pub name: String,
    /// Instructions for how to extract data from conversations.
    #[serde(default)]
    pub prompt: String,
    /// Array of field definitions specifying what data to extract.
    #[serde(default)]
    pub fields: Vec<ExtractionField>,
}

impl ExtractionSchema {
    pub fn builder() -> ExtractionSchemaBuilder {
        <ExtractionSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractionSchemaBuilder {
    id: Option<String>,
    name: Option<String>,
    prompt: Option<String>,
    fields: Option<Vec<ExtractionField>>,
}

impl ExtractionSchemaBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: impl Into<String>) -> Self {
        self.prompt = Some(value.into());
        self
    }

    pub fn fields(mut self, value: Vec<ExtractionField>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExtractionSchema`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExtractionSchemaBuilder::id)
    /// - [`name`](ExtractionSchemaBuilder::name)
    /// - [`prompt`](ExtractionSchemaBuilder::prompt)
    /// - [`fields`](ExtractionSchemaBuilder::fields)
    pub fn build(self) -> Result<ExtractionSchema, BuildError> {
        Ok(ExtractionSchema {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}
