pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConversationsOutboundCallResponse {
        OutboundDryRunResponse(OutboundDryRunResponse),

        OutboundCallInitiatedResponse(OutboundCallInitiatedResponse),
}

impl ConversationsOutboundCallResponse {
    pub fn is_outbound_dry_run_response(&self) -> bool {
        matches!(self, Self::OutboundDryRunResponse(_))
    }

    pub fn is_outbound_call_initiated_response(&self) -> bool {
        matches!(self, Self::OutboundCallInitiatedResponse(_))
    }


    pub fn as_outbound_dry_run_response(&self) -> Option<&OutboundDryRunResponse> {
        match self {
                    Self::OutboundDryRunResponse(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_outbound_dry_run_response(self) -> Option<OutboundDryRunResponse> {
        match self {
                    Self::OutboundDryRunResponse(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_outbound_call_initiated_response(&self) -> Option<&OutboundCallInitiatedResponse> {
        match self {
                    Self::OutboundCallInitiatedResponse(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_outbound_call_initiated_response(self) -> Option<OutboundCallInitiatedResponse> {
        match self {
                    Self::OutboundCallInitiatedResponse(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ConversationsOutboundCallResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutboundDryRunResponse(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::OutboundCallInitiatedResponse(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
