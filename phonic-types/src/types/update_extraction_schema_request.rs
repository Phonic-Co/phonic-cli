pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateExtractionSchemaRequest {
    /// A name for the extraction schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Instructions for how to extract data from conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    /// Array of field definitions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<ExtractionField>>,
    /// The name of the project containing the extraction schema. Only used when `nameOrId` is a name.
    #[serde(skip)]
    pub project: Option<String>,
}

impl UpdateExtractionSchemaRequest {
    pub fn builder() -> UpdateExtractionSchemaRequestBuilder {
        <UpdateExtractionSchemaRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateExtractionSchemaRequestBuilder {
    name: Option<String>,
    prompt: Option<String>,
    fields: Option<Vec<ExtractionField>>,
    project: Option<String>,
}

impl UpdateExtractionSchemaRequestBuilder {
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

    /// Consumes the builder and constructs a [`UpdateExtractionSchemaRequest`].
    pub fn build(self) -> Result<UpdateExtractionSchemaRequest, BuildError> {
        Ok(UpdateExtractionSchemaRequest {
            name: self.name,
            prompt: self.prompt,
            fields: self.fields,
            project: self.project,
        })
    }
}

