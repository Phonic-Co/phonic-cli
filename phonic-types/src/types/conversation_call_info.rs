pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Phone call metadata. `null` for non-phone call conversations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationCallInfo {
    /// Caller phone number in E.164 format.
    #[serde(default)]
    pub from_phone_number: String,
    /// Callee phone number in E.164 format.
    #[serde(default)]
    pub to_phone_number: String,
    /// Twilio Call SID. Only present for user SIP trunking calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twilio_call_sid: Option<String>,
}

impl ConversationCallInfo {
    pub fn builder() -> ConversationCallInfoBuilder {
        <ConversationCallInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationCallInfoBuilder {
    from_phone_number: Option<String>,
    to_phone_number: Option<String>,
    twilio_call_sid: Option<String>,
}

impl ConversationCallInfoBuilder {
    pub fn from_phone_number(mut self, value: impl Into<String>) -> Self {
        self.from_phone_number = Some(value.into());
        self
    }

    pub fn to_phone_number(mut self, value: impl Into<String>) -> Self {
        self.to_phone_number = Some(value.into());
        self
    }

    pub fn twilio_call_sid(mut self, value: impl Into<String>) -> Self {
        self.twilio_call_sid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationCallInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_phone_number`](ConversationCallInfoBuilder::from_phone_number)
    /// - [`to_phone_number`](ConversationCallInfoBuilder::to_phone_number)
    pub fn build(self) -> Result<ConversationCallInfo, BuildError> {
        Ok(ConversationCallInfo {
            from_phone_number: self.from_phone_number.ok_or_else(|| BuildError::missing_field("from_phone_number"))?,
            to_phone_number: self.to_phone_number.ok_or_else(|| BuildError::missing_field("to_phone_number"))?,
            twilio_call_sid: self.twilio_call_sid,
        })
    }
}
