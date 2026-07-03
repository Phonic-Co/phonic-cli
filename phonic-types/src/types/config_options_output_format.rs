pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Audio output format
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConfigOptionsOutputFormat {
    Pcm44100,
    Pcm24000,
    Pcm16000,
    Pcm8000,
    Mulaw8000,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConfigOptionsOutputFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pcm44100 => serializer.serialize_str("pcm_44100"),
            Self::Pcm24000 => serializer.serialize_str("pcm_24000"),
            Self::Pcm16000 => serializer.serialize_str("pcm_16000"),
            Self::Pcm8000 => serializer.serialize_str("pcm_8000"),
            Self::Mulaw8000 => serializer.serialize_str("mulaw_8000"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConfigOptionsOutputFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pcm_44100" => Ok(Self::Pcm44100),
            "pcm_24000" => Ok(Self::Pcm24000),
            "pcm_16000" => Ok(Self::Pcm16000),
            "pcm_8000" => Ok(Self::Pcm8000),
            "mulaw_8000" => Ok(Self::Mulaw8000),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConfigOptionsOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pcm44100 => write!(f, "pcm_44100"),
            Self::Pcm24000 => write!(f, "pcm_24000"),
            Self::Pcm16000 => write!(f, "pcm_16000"),
            Self::Pcm8000 => write!(f, "pcm_8000"),
            Self::Mulaw8000 => write!(f, "mulaw_8000"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
