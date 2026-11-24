pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ConflictErrorBody {
        SipCallError(SipCallError),

        BasicError(BasicError),
}

impl ConflictErrorBody {
    pub fn is_sip_call_error(&self) -> bool {
        matches!(self, Self::SipCallError(_))
    }

    pub fn is_basic_error(&self) -> bool {
        matches!(self, Self::BasicError(_))
    }


    pub fn as_sip_call_error(&self) -> Option<&SipCallError> {
        match self {
                    Self::SipCallError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_sip_call_error(self) -> Option<SipCallError> {
        match self {
                    Self::SipCallError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_basic_error(&self) -> Option<&BasicError> {
        match self {
                    Self::BasicError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_basic_error(self) -> Option<BasicError> {
        match self {
                    Self::BasicError(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ConflictErrorBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SipCallError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::BasicError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
