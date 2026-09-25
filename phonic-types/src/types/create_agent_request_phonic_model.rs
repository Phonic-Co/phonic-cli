pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The Phonic speech-to-speech model to generate with. Omit it to use the current default model.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateAgentRequestPhonicModel {
    PhonicV05,
    PhonicV1,
    PhonicV11,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateAgentRequestPhonicModel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PhonicV05 => serializer.serialize_str("phonic_v0_5"),
            Self::PhonicV1 => serializer.serialize_str("phonic_v1"),
            Self::PhonicV11 => serializer.serialize_str("phonic_v1_1"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateAgentRequestPhonicModel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "phonic_v0_5" => Ok(Self::PhonicV05),
            "phonic_v1" => Ok(Self::PhonicV1),
            "phonic_v1_1" => Ok(Self::PhonicV11),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateAgentRequestPhonicModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PhonicV05 => write!(f, "phonic_v0_5"),
            Self::PhonicV1 => write!(f, "phonic_v1"),
            Self::PhonicV11 => write!(f, "phonic_v1_1"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
