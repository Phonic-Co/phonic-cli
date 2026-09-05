pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GenerateResponsesRequest {
    /// The system prompt the assistant should follow.
    #[serde(default)]
    pub system_prompt: String,
    /// ID of the voice the assistant would speak with. It shapes how the responses are worded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// ISO 639-1 language code that sets the assistant's default language to recognize and speak.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_language: Option<LanguageCode>,
    /// Array of additional ISO 639-1 language codes that the assistant should be able to recognize and speak. Should not include `default_language`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_languages: Option<Vec<LanguageCode>>,
    /// The conversation so far, in order. Must contain at least one item.
    #[serde(default)]
    pub input: Vec<ResponsesInputItem>,
    /// The tools the assistant may call, defined inline. Names must be unique and cannot be one of the names Phonic reserves for its built-in tools.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_definitions: Option<Vec<ResponsesToolDefinition>>,
    /// Number of alternative responses to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_responses: Option<i64>,
}

impl GenerateResponsesRequest {
    pub fn builder() -> GenerateResponsesRequestBuilder {
        <GenerateResponsesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateResponsesRequestBuilder {
    system_prompt: Option<String>,
    voice_id: Option<String>,
    default_language: Option<LanguageCode>,
    additional_languages: Option<Vec<LanguageCode>>,
    input: Option<Vec<ResponsesInputItem>>,
    tool_definitions: Option<Vec<ResponsesToolDefinition>>,
    num_responses: Option<i64>,
}

impl GenerateResponsesRequestBuilder {
    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn default_language(mut self, value: LanguageCode) -> Self {
        self.default_language = Some(value);
        self
    }

    pub fn additional_languages(mut self, value: Vec<LanguageCode>) -> Self {
        self.additional_languages = Some(value);
        self
    }

    pub fn input(mut self, value: Vec<ResponsesInputItem>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn tool_definitions(mut self, value: Vec<ResponsesToolDefinition>) -> Self {
        self.tool_definitions = Some(value);
        self
    }

    pub fn num_responses(mut self, value: i64) -> Self {
        self.num_responses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GenerateResponsesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`system_prompt`](GenerateResponsesRequestBuilder::system_prompt)
    /// - [`input`](GenerateResponsesRequestBuilder::input)
    pub fn build(self) -> Result<GenerateResponsesRequest, BuildError> {
        Ok(GenerateResponsesRequest {
            system_prompt: self.system_prompt.ok_or_else(|| BuildError::missing_field("system_prompt"))?,
            voice_id: self.voice_id,
            default_language: self.default_language,
            additional_languages: self.additional_languages,
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
            tool_definitions: self.tool_definitions,
            num_responses: self.num_responses,
        })
    }
}

