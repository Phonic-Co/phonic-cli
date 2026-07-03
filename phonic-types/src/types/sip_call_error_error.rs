pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SipCallErrorError {
    /// Human-readable error message.
    #[serde(default)]
    pub message: String,
    /// Machine-readable error code.
    pub code: SipCallErrorErrorCode,
    /// The SIP response status code from the carrier (e.g. 403, 486, 603).
    #[serde(default)]
    pub sip_status_code: i64,
    /// The SIP response reason phrase from the carrier (e.g. "Network Blocked", "Trunk CPS limit exceeded").
    #[serde(default)]
    pub sip_status: String,
}

impl SipCallErrorError {
    pub fn builder() -> SipCallErrorErrorBuilder {
        <SipCallErrorErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipCallErrorErrorBuilder {
    message: Option<String>,
    code: Option<SipCallErrorErrorCode>,
    sip_status_code: Option<i64>,
    sip_status: Option<String>,
}

impl SipCallErrorErrorBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn code(mut self, value: SipCallErrorErrorCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn sip_status_code(mut self, value: i64) -> Self {
        self.sip_status_code = Some(value);
        self
    }

    pub fn sip_status(mut self, value: impl Into<String>) -> Self {
        self.sip_status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SipCallErrorError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](SipCallErrorErrorBuilder::message)
    /// - [`code`](SipCallErrorErrorBuilder::code)
    /// - [`sip_status_code`](SipCallErrorErrorBuilder::sip_status_code)
    /// - [`sip_status`](SipCallErrorErrorBuilder::sip_status)
    pub fn build(self) -> Result<SipCallErrorError, BuildError> {
        Ok(SipCallErrorError {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            sip_status_code: self.sip_status_code.ok_or_else(|| BuildError::missing_field("sip_status_code"))?,
            sip_status: self.sip_status.ok_or_else(|| BuildError::missing_field("sip_status"))?,
        })
    }
}
