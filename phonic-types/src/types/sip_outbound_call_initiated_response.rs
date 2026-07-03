pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SipOutboundCallInitiatedResponse {
    /// The ID of the created conversation.
    #[serde(default)]
    pub conversation_id: String,
    /// The Twilio Call SID.
    #[serde(default)]
    pub twilio_call_sid: String,
    /// Always false when a call was placed.
    pub dry_run: bool,
}

impl SipOutboundCallInitiatedResponse {
    pub fn builder() -> SipOutboundCallInitiatedResponseBuilder {
        <SipOutboundCallInitiatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipOutboundCallInitiatedResponseBuilder {
    conversation_id: Option<String>,
    twilio_call_sid: Option<String>,
    dry_run: Option<bool>,
}

impl SipOutboundCallInitiatedResponseBuilder {
    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn twilio_call_sid(mut self, value: impl Into<String>) -> Self {
        self.twilio_call_sid = Some(value.into());
        self
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SipOutboundCallInitiatedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_id`](SipOutboundCallInitiatedResponseBuilder::conversation_id)
    /// - [`twilio_call_sid`](SipOutboundCallInitiatedResponseBuilder::twilio_call_sid)
    /// - [`dry_run`](SipOutboundCallInitiatedResponseBuilder::dry_run)
    pub fn build(self) -> Result<SipOutboundCallInitiatedResponse, BuildError> {
        Ok(SipOutboundCallInitiatedResponse {
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            twilio_call_sid: self.twilio_call_sid.ok_or_else(|| BuildError::missing_field("twilio_call_sid"))?,
            dry_run: self.dry_run.ok_or_else(|| BuildError::missing_field("dry_run"))?,
        })
    }
}
