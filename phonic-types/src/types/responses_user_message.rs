pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Something the user said.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResponsesUserMessage {
    pub role: String,
    /// What the user said.
    #[serde(default)]
    pub text: String,
}

impl ResponsesUserMessage {
    pub fn builder() -> ResponsesUserMessageBuilder {
        <ResponsesUserMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesUserMessageBuilder {
    role: Option<String>,
    text: Option<String>,
}

impl ResponsesUserMessageBuilder {
    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponsesUserMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`role`](ResponsesUserMessageBuilder::role)
    /// - [`text`](ResponsesUserMessageBuilder::text)
    pub fn build(self) -> Result<ResponsesUserMessage, BuildError> {
        Ok(ResponsesUserMessage {
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
