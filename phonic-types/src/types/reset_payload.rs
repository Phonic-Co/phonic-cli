pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResetPayload {
    pub r#type: String,
    #[serde(default)]
    pub config: ConfigOptions,
}

impl ResetPayload {
    pub fn builder() -> ResetPayloadBuilder {
        <ResetPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResetPayloadBuilder {
    r#type: Option<String>,
    config: Option<ConfigOptions>,
}

impl ResetPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn config(mut self, value: ConfigOptions) -> Self {
        self.config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResetPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ResetPayloadBuilder::r#type)
    /// - [`config`](ResetPayloadBuilder::config)
    pub fn build(self) -> Result<ResetPayload, BuildError> {
        Ok(ResetPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            config: self.config.ok_or_else(|| BuildError::missing_field("config"))?,
        })
    }
}
