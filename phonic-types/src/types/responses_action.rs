pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum ResponsesAction {
        #[serde(rename = "assistant_chose_not_to_respond")]
        #[non_exhaustive]
        AssistantChoseNotToRespond {
            #[serde(skip_serializing_if = "Option::is_none")]
            respond_after_sec: Option<f64>,
        },

        #[serde(rename = "assistant_ended_conversation")]
        #[non_exhaustive]
        AssistantEndedConversation {},

        #[serde(rename = "dtmf")]
        #[non_exhaustive]
        Dtmf {
            #[serde(default)]
            digits: String,
        },

        #[serde(rename = "transfer_to_phone_number")]
        #[non_exhaustive]
        TransferToPhoneNumber {
            #[serde(skip_serializing_if = "Option::is_none")]
            phone_number: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            sip: Option<TransferSip>,
            #[serde(default)]
            detect_voicemail: bool,
            #[serde(default)]
            use_agent_phone_number: bool,
            #[serde(default)]
            keep_listening: bool,
            on_transfer_no_answer: ResponsesTransferToPhoneNumberActionOnTransferNoAnswer,
            #[serde(skip_serializing_if = "Option::is_none")]
            dtmf: Option<String>,
        },

        #[serde(rename = "transfer_to_agent")]
        #[non_exhaustive]
        TransferToAgent {
            #[serde(default)]
            agent: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ResponsesAction {
    pub fn assistant_chose_not_to_respond() -> Self {
        Self::AssistantChoseNotToRespond { respond_after_sec: None }
    }

    pub fn assistant_ended_conversation() -> Self {
        Self::AssistantEndedConversation {}
    }

    pub fn dtmf(digits: String) -> Self {
        Self::Dtmf { digits }
    }

    pub fn transfer_to_phone_number(detect_voicemail: bool, use_agent_phone_number: bool, keep_listening: bool, on_transfer_no_answer: ResponsesTransferToPhoneNumberActionOnTransferNoAnswer) -> Self {
        Self::TransferToPhoneNumber { phone_number: None, sip: None, detect_voicemail, use_agent_phone_number, keep_listening, on_transfer_no_answer, dtmf: None }
    }

    pub fn transfer_to_agent(agent: String) -> Self {
        Self::TransferToAgent { agent }
    }

    pub fn assistant_chose_not_to_respond_with_respond_after_sec(respond_after_sec: f64) -> Self {
        Self::AssistantChoseNotToRespond { respond_after_sec: Some(respond_after_sec) }
    }

    pub fn transfer_to_phone_number_with_phone_number(phone_number: String, sip: Option<TransferSip>, detect_voicemail: bool, use_agent_phone_number: bool, keep_listening: bool, on_transfer_no_answer: ResponsesTransferToPhoneNumberActionOnTransferNoAnswer, dtmf: Option<String>) -> Self {
        Self::TransferToPhoneNumber { phone_number: Some(phone_number), sip, detect_voicemail, use_agent_phone_number, keep_listening, on_transfer_no_answer, dtmf }
    }

    pub fn transfer_to_phone_number_with_sip(phone_number: Option<String>, sip: TransferSip, detect_voicemail: bool, use_agent_phone_number: bool, keep_listening: bool, on_transfer_no_answer: ResponsesTransferToPhoneNumberActionOnTransferNoAnswer, dtmf: Option<String>) -> Self {
        Self::TransferToPhoneNumber { phone_number, sip: Some(sip), detect_voicemail, use_agent_phone_number, keep_listening, on_transfer_no_answer, dtmf }
    }

    pub fn transfer_to_phone_number_with_dtmf(phone_number: Option<String>, sip: Option<TransferSip>, detect_voicemail: bool, use_agent_phone_number: bool, keep_listening: bool, on_transfer_no_answer: ResponsesTransferToPhoneNumberActionOnTransferNoAnswer, dtmf: String) -> Self {
        Self::TransferToPhoneNumber { phone_number, sip, detect_voicemail, use_agent_phone_number, keep_listening, on_transfer_no_answer, dtmf: Some(dtmf) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
