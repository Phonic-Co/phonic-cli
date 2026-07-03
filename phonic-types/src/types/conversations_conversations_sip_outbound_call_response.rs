pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConversationsSipOutboundCallResponse {
        SipOutboundDryRunResponse(SipOutboundDryRunResponse),

        SipOutboundCallInitiatedResponse(SipOutboundCallInitiatedResponse),
}

impl ConversationsSipOutboundCallResponse {
    pub fn is_sip_outbound_dry_run_response(&self) -> bool {
        matches!(self, Self::SipOutboundDryRunResponse(_))
    }

    pub fn is_sip_outbound_call_initiated_response(&self) -> bool {
        matches!(self, Self::SipOutboundCallInitiatedResponse(_))
    }


    pub fn as_sip_outbound_dry_run_response(&self) -> Option<&SipOutboundDryRunResponse> {
        match self {
                    Self::SipOutboundDryRunResponse(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_sip_outbound_dry_run_response(self) -> Option<SipOutboundDryRunResponse> {
        match self {
                    Self::SipOutboundDryRunResponse(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_sip_outbound_call_initiated_response(&self) -> Option<&SipOutboundCallInitiatedResponse> {
        match self {
                    Self::SipOutboundCallInitiatedResponse(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_sip_outbound_call_initiated_response(self) -> Option<SipOutboundCallInitiatedResponse> {
        match self {
                    Self::SipOutboundCallInitiatedResponse(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ConversationsSipOutboundCallResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SipOutboundDryRunResponse(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::SipOutboundCallInitiatedResponse(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
