pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// SIP trunk settings for the outbound trunk created for this call.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsSipOutboundCallRequestSip {
    /// SIP signaling transport used for this call. The SIP address must not carry a `;transport=` parameter; set it here instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport: Option<ConversationsSipOutboundCallRequestSipTransport>,
    /// Whether media (SRTP) encryption is used on this call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_encryption: Option<ConversationsSipOutboundCallRequestSipMediaEncryption>,
}

impl ConversationsSipOutboundCallRequestSip {
    pub fn builder() -> ConversationsSipOutboundCallRequestSipBuilder {
        <ConversationsSipOutboundCallRequestSipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsSipOutboundCallRequestSipBuilder {
    transport: Option<ConversationsSipOutboundCallRequestSipTransport>,
    media_encryption: Option<ConversationsSipOutboundCallRequestSipMediaEncryption>,
}

impl ConversationsSipOutboundCallRequestSipBuilder {
    pub fn transport(mut self, value: ConversationsSipOutboundCallRequestSipTransport) -> Self {
        self.transport = Some(value);
        self
    }

    pub fn media_encryption(mut self, value: ConversationsSipOutboundCallRequestSipMediaEncryption) -> Self {
        self.media_encryption = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsSipOutboundCallRequestSip`].
    pub fn build(self) -> Result<ConversationsSipOutboundCallRequestSip, BuildError> {
        Ok(ConversationsSipOutboundCallRequestSip {
            transport: self.transport,
            media_encryption: self.media_encryption,
        })
    }
}
