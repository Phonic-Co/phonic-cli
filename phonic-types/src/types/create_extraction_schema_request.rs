pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateExtractionSchemaRequest {
    /// A name for the extraction schema.
    #[serde(default)]
    pub name: String,
    /// Instructions for how to extract data from conversations.
    #[serde(default)]
    pub prompt: String,
    /// Array of field definitions.
    #[serde(default)]
    pub fields: Vec<ExtractionField>,
    /// The name of the project to create the extraction schema in.
    #[serde(skip)]
    pub project: Option<String>,
}

impl CreateExtractionSchemaRequest {
    pub fn builder() -> CreateExtractionSchemaRequestBuilder {
        <CreateExtractionSchemaRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateExtractionSchemaRequestBuilder {
    name: Option<String>,
    prompt: Option<String>,
    fields: Option<Vec<ExtractionField>>,
    project: Option<String>,
}

impl CreateExtractionSchemaRequestBuilder {
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

    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateExtractionSchemaRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateExtractionSchemaRequestBuilder::name)
    /// - [`prompt`](CreateExtractionSchemaRequestBuilder::prompt)
    /// - [`fields`](CreateExtractionSchemaRequestBuilder::fields)
    pub fn build(self) -> Result<CreateExtractionSchemaRequest, BuildError> {
        Ok(CreateExtractionSchemaRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            fields: self.fields.ok_or_else(|| BuildError::missing_field("fields"))?,
            project: self.project,
        })
    }
}

