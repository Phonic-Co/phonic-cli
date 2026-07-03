pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ValidationError {
    #[serde(default)]
    pub error: ValidationErrorError,
    /// Parameter-specific validation errors
    #[serde(default)]
    pub param_errors: HashMap<String, String>,
}

impl ValidationError {
    pub fn builder() -> ValidationErrorBuilder {
        <ValidationErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidationErrorBuilder {
    error: Option<ValidationErrorError>,
    param_errors: Option<HashMap<String, String>>,
}

impl ValidationErrorBuilder {
    pub fn error(mut self, value: ValidationErrorError) -> Self {
        self.error = Some(value);
        self
    }

    pub fn param_errors(mut self, value: HashMap<String, String>) -> Self {
        self.param_errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ValidationError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](ValidationErrorBuilder::error)
    /// - [`param_errors`](ValidationErrorBuilder::param_errors)
    pub fn build(self) -> Result<ValidationError, BuildError> {
        Ok(ValidationError {
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
            param_errors: self.param_errors.ok_or_else(|| BuildError::missing_field("param_errors"))?,
        })
    }
}
