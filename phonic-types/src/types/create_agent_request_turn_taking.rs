pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How long the agent waits after the user stops speaking before taking its turn. `"eager"` responds fastest; `"very_patient"` waits the longest.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateAgentRequestTurnTaking {
    Eager,
    Standard,
    Relaxed,
    Measured,
    Patient,
    VeryPatient,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateAgentRequestTurnTaking {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Eager => serializer.serialize_str("eager"),
            Self::Standard => serializer.serialize_str("standard"),
            Self::Relaxed => serializer.serialize_str("relaxed"),
            Self::Measured => serializer.serialize_str("measured"),
            Self::Patient => serializer.serialize_str("patient"),
            Self::VeryPatient => serializer.serialize_str("very_patient"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateAgentRequestTurnTaking {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eager" => Ok(Self::Eager),
            "standard" => Ok(Self::Standard),
            "relaxed" => Ok(Self::Relaxed),
            "measured" => Ok(Self::Measured),
            "patient" => Ok(Self::Patient),
            "very_patient" => Ok(Self::VeryPatient),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateAgentRequestTurnTaking {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eager => write!(f, "eager"),
            Self::Standard => write!(f, "standard"),
            Self::Relaxed => write!(f, "relaxed"),
            Self::Measured => write!(f, "measured"),
            Self::Patient => write!(f, "patient"),
            Self::VeryPatient => write!(f, "very_patient"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
