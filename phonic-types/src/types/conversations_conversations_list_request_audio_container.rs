pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationsListRequestAudioContainer {
    WavGz,
    Wav,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationsListRequestAudioContainer {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WavGz => serializer.serialize_str("wav.gz"),
            Self::Wav => serializer.serialize_str("wav"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationsListRequestAudioContainer {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "wav.gz" => Ok(Self::WavGz),
            "wav" => Ok(Self::Wav),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationsListRequestAudioContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WavGz => write!(f, "wav.gz"),
            Self::Wav => write!(f, "wav"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
