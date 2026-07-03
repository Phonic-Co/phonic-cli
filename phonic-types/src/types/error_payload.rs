pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorPayload {
    pub r#type: String,
    #[serde(default)]
    pub error: ErrorPayloadError,
    /// Parameter-specific validation errors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param_errors: Option<HashMap<String, String>>,
}

impl ErrorPayload {
    pub fn builder() -> ErrorPayloadBuilder {
        <ErrorPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorPayloadBuilder {
    r#type: Option<String>,
    error: Option<ErrorPayloadError>,
    param_errors: Option<HashMap<String, String>>,
}

impl ErrorPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn error(mut self, value: ErrorPayloadError) -> Self {
        self.error = Some(value);
        self
    }

    pub fn param_errors(mut self, value: HashMap<String, String>) -> Self {
        self.param_errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ErrorPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ErrorPayloadBuilder::r#type)
    /// - [`error`](ErrorPayloadBuilder::error)
    pub fn build(self) -> Result<ErrorPayload, BuildError> {
        Ok(ErrorPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
            param_errors: self.param_errors,
        })
    }
}
