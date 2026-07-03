pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfigPayload {
    #[serde(flatten)]
    pub config_options_fields: ConfigOptions,
    pub r#type: String,
}

impl ConfigPayload {
    pub fn builder() -> ConfigPayloadBuilder {
        <ConfigPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigPayloadBuilder {
    config_options_fields: Option<ConfigOptions>,
    r#type: Option<String>,
}

impl ConfigPayloadBuilder {
    pub fn config_options_fields(mut self, value: ConfigOptions) -> Self {
        self.config_options_fields = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConfigPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`config_options_fields`](ConfigPayloadBuilder::config_options_fields)
    /// - [`r#type`](ConfigPayloadBuilder::r#type)
    pub fn build(self) -> Result<ConfigPayload, BuildError> {
        Ok(ConfigPayload {
            config_options_fields: self.config_options_fields.ok_or_else(|| BuildError::missing_field("config_options_fields"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
