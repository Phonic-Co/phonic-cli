pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationExtractionSchema {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    /// Whether the extraction schema has been deleted.
    #[serde(rename = "isDeleted")]
    #[serde(default)]
    pub is_deleted: bool,
}

impl ConversationExtractionSchema {
    pub fn builder() -> ConversationExtractionSchemaBuilder {
        <ConversationExtractionSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationExtractionSchemaBuilder {
    id: Option<String>,
    name: Option<String>,
    is_deleted: Option<bool>,
}

impl ConversationExtractionSchemaBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_deleted(mut self, value: bool) -> Self {
        self.is_deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationExtractionSchema`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationExtractionSchemaBuilder::id)
    /// - [`name`](ConversationExtractionSchemaBuilder::name)
    /// - [`is_deleted`](ConversationExtractionSchemaBuilder::is_deleted)
    pub fn build(self) -> Result<ConversationExtractionSchema, BuildError> {
        Ok(ConversationExtractionSchema {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_deleted: self.is_deleted.ok_or_else(|| BuildError::missing_field("is_deleted"))?,
        })
    }
}
