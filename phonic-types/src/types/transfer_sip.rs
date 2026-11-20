pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A SIP destination to transfer calls to, used instead of a phone number.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferSip {
    /// The SIP URI to transfer calls to, in the form `sip:user@host[:port][;transport=udp|tcp|tls]` or `sips:user@host[:port]`. `transport` is the only URI parameter allowed.
    #[serde(default)]
    pub uri: String,
    /// Custom SIP headers to send with the transfer. Header names must start with `X-`, and values cannot contain line breaks. Headers require a bridged transfer (keep_listening true, or DTMF set), since a SIP REFER transfer can't carry them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
}

impl TransferSip {
    pub fn builder() -> TransferSipBuilder {
        <TransferSipBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferSipBuilder {
    uri: Option<String>,
    headers: Option<HashMap<String, String>>,
}

impl TransferSipBuilder {
    pub fn uri(mut self, value: impl Into<String>) -> Self {
        self.uri = Some(value.into());
        self
    }

    pub fn headers(mut self, value: HashMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferSip`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uri`](TransferSipBuilder::uri)
    pub fn build(self) -> Result<TransferSip, BuildError> {
        Ok(TransferSip {
            uri: self.uri.ok_or_else(|| BuildError::missing_field("uri"))?,
            headers: self.headers,
        })
    }
}
