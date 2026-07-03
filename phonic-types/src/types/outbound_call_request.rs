pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OutboundCallRequest {
    /// The phone number to call in E.164 format.
    #[serde(default)]
    pub to_phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<OutboundCallConfig>,
    /// If true, validates the outbound call setup without placing a call. Returns HTTP 200 with `conversation_id` set to null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl OutboundCallRequest {
    pub fn builder() -> OutboundCallRequestBuilder {
        <OutboundCallRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutboundCallRequestBuilder {
    to_phone_number: Option<String>,
    config: Option<OutboundCallConfig>,
    dry_run: Option<bool>,
}

impl OutboundCallRequestBuilder {
    pub fn to_phone_number(mut self, value: impl Into<String>) -> Self {
        self.to_phone_number = Some(value.into());
        self
    }

    pub fn config(mut self, value: OutboundCallConfig) -> Self {
        self.config = Some(value);
        self
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OutboundCallRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`to_phone_number`](OutboundCallRequestBuilder::to_phone_number)
    pub fn build(self) -> Result<OutboundCallRequest, BuildError> {
        Ok(OutboundCallRequest {
            to_phone_number: self.to_phone_number.ok_or_else(|| BuildError::missing_field("to_phone_number"))?,
            config: self.config,
            dry_run: self.dry_run,
        })
    }
}

