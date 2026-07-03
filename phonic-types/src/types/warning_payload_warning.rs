pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WarningPayloadWarning {
    /// Warning message
    #[serde(default)]
    pub message: String,
    /// Warning code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl WarningPayloadWarning {
    pub fn builder() -> WarningPayloadWarningBuilder {
        <WarningPayloadWarningBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WarningPayloadWarningBuilder {
    message: Option<String>,
    code: Option<String>,
}

impl WarningPayloadWarningBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WarningPayloadWarning`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](WarningPayloadWarningBuilder::message)
    pub fn build(self) -> Result<WarningPayloadWarning, BuildError> {
        Ok(WarningPayloadWarning {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            code: self.code,
        })
    }
}
