pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationsExtractDataResponse {
    /// The extracted data according to the schema.
    #[serde(default)]
    pub result: HashMap<String, serde_json::Value>,
}

impl ConversationsExtractDataResponse {
    pub fn builder() -> ConversationsExtractDataResponseBuilder {
        <ConversationsExtractDataResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsExtractDataResponseBuilder {
    result: Option<HashMap<String, serde_json::Value>>,
}

impl ConversationsExtractDataResponseBuilder {
    pub fn result(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsExtractDataResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result`](ConversationsExtractDataResponseBuilder::result)
    pub fn build(self) -> Result<ConversationsExtractDataResponse, BuildError> {
        Ok(ConversationsExtractDataResponse {
            result: self.result.ok_or_else(|| BuildError::missing_field("result"))?,
        })
    }
}
