pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateAgentRequestPhoneNumber {
    AssignAutomatically,
    Custom,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateAgentRequestPhoneNumber {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AssignAutomatically => serializer.serialize_str("assign-automatically"),
            Self::Custom => serializer.serialize_str("custom"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateAgentRequestPhoneNumber {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "assign-automatically" => Ok(Self::AssignAutomatically),
            "custom" => Ok(Self::Custom),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateAgentRequestPhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AssignAutomatically => write!(f, "assign-automatically"),
            Self::Custom => write!(f, "custom"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
