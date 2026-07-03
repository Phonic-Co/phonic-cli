pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationExtraction {
    /// The extraction ID.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub schema: ConversationExtractionSchema,
    /// The extracted data.
    #[serde(default)]
    pub result: HashMap<String, serde_json::Value>,
    /// When the extraction was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl ConversationExtraction {
    pub fn builder() -> ConversationExtractionBuilder {
        <ConversationExtractionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationExtractionBuilder {
    id: Option<String>,
    schema: Option<ConversationExtractionSchema>,
    result: Option<HashMap<String, serde_json::Value>>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl ConversationExtractionBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn schema(mut self, value: ConversationExtractionSchema) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn result(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.result = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationExtraction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationExtractionBuilder::id)
    /// - [`schema`](ConversationExtractionBuilder::schema)
    /// - [`result`](ConversationExtractionBuilder::result)
    /// - [`created_at`](ConversationExtractionBuilder::created_at)
    pub fn build(self) -> Result<ConversationExtraction, BuildError> {
        Ok(ConversationExtraction {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            schema: self.schema.ok_or_else(|| BuildError::missing_field("schema"))?,
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
