pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GenerateResponsesResponse {
    /// The generated responses - always `num_responses` of them.
    #[serde(default)]
    pub responses: Vec<GeneratedResponse>,
}

impl GenerateResponsesResponse {
    pub fn builder() -> GenerateResponsesResponseBuilder {
        <GenerateResponsesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateResponsesResponseBuilder {
    responses: Option<Vec<GeneratedResponse>>,
}

impl GenerateResponsesResponseBuilder {
    pub fn responses(mut self, value: Vec<GeneratedResponse>) -> Self {
        self.responses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GenerateResponsesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`responses`](GenerateResponsesResponseBuilder::responses)
    pub fn build(self) -> Result<GenerateResponsesResponse, BuildError> {
        Ok(GenerateResponsesResponse {
            responses: self.responses.ok_or_else(|| BuildError::missing_field("responses"))?,
        })
    }
}
