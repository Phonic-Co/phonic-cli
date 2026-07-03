pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExtractionField {
    /// The field name.
    #[serde(default)]
    pub name: String,
    /// The field type.
    pub r#type: ExtractionFieldType,
    /// Description of the field. May be null, but the property must be present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ExtractionField {
    pub fn builder() -> ExtractionFieldBuilder {
        <ExtractionFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractionFieldBuilder {
    name: Option<String>,
    r#type: Option<ExtractionFieldType>,
    description: Option<String>,
}

impl ExtractionFieldBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ExtractionFieldType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExtractionField`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ExtractionFieldBuilder::name)
    /// - [`r#type`](ExtractionFieldBuilder::r#type)
    pub fn build(self) -> Result<ExtractionField, BuildError> {
        Ok(ExtractionField {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            description: self.description,
        })
    }
}
