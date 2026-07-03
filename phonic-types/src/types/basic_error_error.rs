pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BasicErrorError {
    /// Error message
    #[serde(default)]
    pub message: String,
    /// Error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl BasicErrorError {
    pub fn builder() -> BasicErrorErrorBuilder {
        <BasicErrorErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BasicErrorErrorBuilder {
    message: Option<String>,
    code: Option<String>,
}

impl BasicErrorErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BasicErrorError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](BasicErrorErrorBuilder::message)
    pub fn build(self) -> Result<BasicErrorError, BuildError> {
        Ok(BasicErrorError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            code: self.code,
        })
    }
}
