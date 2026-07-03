pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WarningPayload {
    pub r#type: String,
    #[serde(default)]
    pub warning: WarningPayloadWarning,
}

impl WarningPayload {
    pub fn builder() -> WarningPayloadBuilder {
        <WarningPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WarningPayloadBuilder {
    r#type: Option<String>,
    warning: Option<WarningPayloadWarning>,
}

impl WarningPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn warning(mut self, value: WarningPayloadWarning) -> Self {
        self.warning = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WarningPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](WarningPayloadBuilder::r#type)
    /// - [`warning`](WarningPayloadBuilder::warning)
    pub fn build(self) -> Result<WarningPayload, BuildError> {
        Ok(WarningPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            warning: self.warning.ok_or_else(|| BuildError::missing_field("warning"))?,
        })
    }
}
