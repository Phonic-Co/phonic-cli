pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidationErrorError {
    /// Error message
    #[serde(default)]
    pub message: String,
}

impl ValidationErrorError {
    pub fn builder() -> ValidationErrorErrorBuilder {
        <ValidationErrorErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidationErrorErrorBuilder {
    message: Option<String>,
}

impl ValidationErrorErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ValidationErrorError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ValidationErrorErrorBuilder::message)
    pub fn build(self) -> Result<ValidationErrorError, BuildError> {
        Ok(ValidationErrorError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
