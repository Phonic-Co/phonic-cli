pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InputTextPayload {
    pub r#type: String,
    /// Detected ISO 639-1 language code of user speech
    #[serde(default)]
    pub language: String,
    /// Transcribed user speech
    #[serde(default)]
    pub text: String,
}

impl InputTextPayload {
    pub fn builder() -> InputTextPayloadBuilder {
        <InputTextPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InputTextPayloadBuilder {
    r#type: Option<String>,
    language: Option<String>,
    text: Option<String>,
}

impl InputTextPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InputTextPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](InputTextPayloadBuilder::r#type)
    /// - [`language`](InputTextPayloadBuilder::language)
    /// - [`text`](InputTextPayloadBuilder::text)
    pub fn build(self) -> Result<InputTextPayload, BuildError> {
        Ok(InputTextPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
