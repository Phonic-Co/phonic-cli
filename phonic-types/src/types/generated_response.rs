pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GeneratedResponse {
    /// The text the assistant would say. Empty when the response only makes tool calls.
    #[serde(default)]
    pub text: String,
    /// Tool calls the assistant would make - note that the tools are not actually called.
    #[serde(default)]
    pub tool_calls: Vec<GeneratedToolCall>,
}

impl GeneratedResponse {
    pub fn builder() -> GeneratedResponseBuilder {
        <GeneratedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneratedResponseBuilder {
    text: Option<String>,
    tool_calls: Option<Vec<GeneratedToolCall>>,
}

impl GeneratedResponseBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn tool_calls(mut self, value: Vec<GeneratedToolCall>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeneratedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](GeneratedResponseBuilder::text)
    /// - [`tool_calls`](GeneratedResponseBuilder::tool_calls)
    pub fn build(self) -> Result<GeneratedResponse, BuildError> {
        Ok(GeneratedResponse {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            tool_calls: self.tool_calls.ok_or_else(|| BuildError::missing_field("tool_calls"))?,
        })
    }
}
