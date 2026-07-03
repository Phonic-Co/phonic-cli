pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The evaluation result.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConversationEvaluationResultResult {
    Successful,
    Unsuccessful,
    Undecided,
    Error,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ConversationEvaluationResultResult {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Successful => serializer.serialize_str("successful"),
            Self::Unsuccessful => serializer.serialize_str("unsuccessful"),
            Self::Undecided => serializer.serialize_str("undecided"),
            Self::Error => serializer.serialize_str("error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ConversationEvaluationResultResult {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "successful" => Ok(Self::Successful),
            "unsuccessful" => Ok(Self::Unsuccessful),
            "undecided" => Ok(Self::Undecided),
            "error" => Ok(Self::Error),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ConversationEvaluationResultResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Successful => write!(f, "successful"),
            Self::Unsuccessful => write!(f, "unsuccessful"),
            Self::Undecided => write!(f, "undecided"),
            Self::Error => write!(f, "error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
