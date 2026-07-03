pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Machine-readable error code.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SipCallErrorErrorCode {
    SipCallFailed,
    SipAuthFailed,
    InvalidPhoneNumber,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SipCallErrorErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SipCallFailed => serializer.serialize_str("sip_call_failed"),
            Self::SipAuthFailed => serializer.serialize_str("sip_auth_failed"),
            Self::InvalidPhoneNumber => serializer.serialize_str("invalid_phone_number"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SipCallErrorErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sip_call_failed" => Ok(Self::SipCallFailed),
            "sip_auth_failed" => Ok(Self::SipAuthFailed),
            "invalid_phone_number" => Ok(Self::InvalidPhoneNumber),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SipCallErrorErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SipCallFailed => write!(f, "sip_call_failed"),
            Self::SipAuthFailed => write!(f, "sip_auth_failed"),
            Self::InvalidPhoneNumber => write!(f, "invalid_phone_number"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
