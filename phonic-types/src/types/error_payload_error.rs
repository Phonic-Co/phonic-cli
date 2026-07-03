pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ErrorPayloadError {
    /// Error message
    #[serde(default)]
    pub message: String,
    /// Error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl ErrorPayloadError {
    pub fn builder() -> ErrorPayloadErrorBuilder {
        <ErrorPayloadErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorPayloadErrorBuilder {
    message: Option<String>,
    code: Option<String>,
}

impl ErrorPayloadErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ErrorPayloadError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ErrorPayloadErrorBuilder::message)
    pub fn build(self) -> Result<ErrorPayloadError, BuildError> {
        Ok(ErrorPayloadError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            code: self.code,
        })
    }
}
