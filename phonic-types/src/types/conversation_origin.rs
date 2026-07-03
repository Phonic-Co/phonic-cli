pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The origin of the conversation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationOrigin {
    Web,
    WebPlayground,
    WebDemo,
    Direct,
    LivekitAgentsPy,
    LivekitAgentsJs,
    SdkPy,
    SdkJs,
    Inbound,
    TelephonyInbound,
    Outbound,
    TelephonyOutbound,
    Replay,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Web => serializer.serialize_str("web"),
            Self::WebPlayground => serializer.serialize_str("web-playground"),
            Self::WebDemo => serializer.serialize_str("web-demo"),
            Self::Direct => serializer.serialize_str("direct"),
            Self::LivekitAgentsPy => serializer.serialize_str("livekit-agents-py"),
            Self::LivekitAgentsJs => serializer.serialize_str("livekit-agents-js"),
            Self::SdkPy => serializer.serialize_str("sdk-py"),
            Self::SdkJs => serializer.serialize_str("sdk-js"),
            Self::Inbound => serializer.serialize_str("inbound"),
            Self::TelephonyInbound => serializer.serialize_str("telephony-inbound"),
            Self::Outbound => serializer.serialize_str("outbound"),
            Self::TelephonyOutbound => serializer.serialize_str("telephony-outbound"),
            Self::Replay => serializer.serialize_str("replay"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "web" => Ok(Self::Web),
            "web-playground" => Ok(Self::WebPlayground),
            "web-demo" => Ok(Self::WebDemo),
            "direct" => Ok(Self::Direct),
            "livekit-agents-py" => Ok(Self::LivekitAgentsPy),
            "livekit-agents-js" => Ok(Self::LivekitAgentsJs),
            "sdk-py" => Ok(Self::SdkPy),
            "sdk-js" => Ok(Self::SdkJs),
            "inbound" => Ok(Self::Inbound),
            "telephony-inbound" => Ok(Self::TelephonyInbound),
            "outbound" => Ok(Self::Outbound),
            "telephony-outbound" => Ok(Self::TelephonyOutbound),
            "replay" => Ok(Self::Replay),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Web => write!(f, "web"),
            Self::WebPlayground => write!(f, "web-playground"),
            Self::WebDemo => write!(f, "web-demo"),
            Self::Direct => write!(f, "direct"),
            Self::LivekitAgentsPy => write!(f, "livekit-agents-py"),
            Self::LivekitAgentsJs => write!(f, "livekit-agents-js"),
            Self::SdkPy => write!(f, "sdk-py"),
            Self::SdkJs => write!(f, "sdk-js"),
            Self::Inbound => write!(f, "inbound"),
            Self::TelephonyInbound => write!(f, "telephony-inbound"),
            Self::Outbound => write!(f, "outbound"),
            Self::TelephonyOutbound => write!(f, "telephony-outbound"),
            Self::Replay => write!(f, "replay"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
