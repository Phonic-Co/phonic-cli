pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConversationsSipOutboundCallRequest {
    /// Caller ID phone number in E.164 format.
    #[serde(default)]
    pub from_phone_number: String,
    /// Destination phone number in E.164 format.
    #[serde(default)]
    pub to_phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<OutboundCallConfig>,
    /// If true, validates the outbound call setup without placing a call. Returns HTTP 200 with `conversation_id` and `twilio_call_sid` set to null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

impl ConversationsSipOutboundCallRequest {
    pub fn builder() -> ConversationsSipOutboundCallRequestBuilder {
        <ConversationsSipOutboundCallRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsSipOutboundCallRequestBuilder {
    from_phone_number: Option<String>,
    to_phone_number: Option<String>,
    config: Option<OutboundCallConfig>,
    dry_run: Option<bool>,
}

impl ConversationsSipOutboundCallRequestBuilder {
    pub fn from_phone_number(mut self, value: impl Into<String>) -> Self {
        self.from_phone_number = Some(value.into());
        self
    }

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

    /// Consumes the builder and constructs a [`ConversationsSipOutboundCallRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_phone_number`](ConversationsSipOutboundCallRequestBuilder::from_phone_number)
    /// - [`to_phone_number`](ConversationsSipOutboundCallRequestBuilder::to_phone_number)
    pub fn build(self) -> Result<ConversationsSipOutboundCallRequest, BuildError> {
        Ok(ConversationsSipOutboundCallRequest {
            from_phone_number: self.from_phone_number.ok_or_else(|| BuildError::missing_field("from_phone_number"))?,
            to_phone_number: self.to_phone_number.ok_or_else(|| BuildError::missing_field("to_phone_number"))?,
            config: self.config,
            dry_run: self.dry_run,
        })
    }
}

