pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// SIP trunk settings for this phone number, applied to both its inbound trunk and the trunk created for each outbound call. Set at creation; remove and re-add the number to change them.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsAddCustomPhoneNumberRequestSip {
    /// SIP signaling transport used for calls on this phone number. The SIP address must not carry a `;transport=` parameter; set it here instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<AgentsAddCustomPhoneNumberRequestSipTransport>,
    /// Whether media (SRTP) encryption is used on calls on this phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_encryption: Option<AgentsAddCustomPhoneNumberRequestSipMediaEncryption>,
}

impl AgentsAddCustomPhoneNumberRequestSip {
    pub fn builder() -> AgentsAddCustomPhoneNumberRequestSipBuilder {
        <AgentsAddCustomPhoneNumberRequestSipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsAddCustomPhoneNumberRequestSipBuilder {
    transport: Option<AgentsAddCustomPhoneNumberRequestSipTransport>,
    media_encryption: Option<AgentsAddCustomPhoneNumberRequestSipMediaEncryption>,
}

impl AgentsAddCustomPhoneNumberRequestSipBuilder {
    pub fn transport(mut self, value: AgentsAddCustomPhoneNumberRequestSipTransport) -> Self {
        self.transport = Some(value);
        self
    }

    pub fn media_encryption(mut self, value: AgentsAddCustomPhoneNumberRequestSipMediaEncryption) -> Self {
        self.media_encryption = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsAddCustomPhoneNumberRequestSip`].
    pub fn build(self) -> Result<AgentsAddCustomPhoneNumberRequestSip, BuildError> {
        Ok(AgentsAddCustomPhoneNumberRequestSip {
            transport: self.transport,
            media_encryption: self.media_encryption,
        })
    }
}
