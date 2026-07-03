pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The parameter type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolParameterType {
    String,
    Integer,
    Number,
    Boolean,
    Array,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolParameterType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::String => serializer.serialize_str("string"),
            Self::Integer => serializer.serialize_str("integer"),
            Self::Number => serializer.serialize_str("number"),
            Self::Boolean => serializer.serialize_str("boolean"),
            Self::Array => serializer.serialize_str("array"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolParameterType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "string" => Ok(Self::String),
            "integer" => Ok(Self::Integer),
            "number" => Ok(Self::Number),
            "boolean" => Ok(Self::Boolean),
            "array" => Ok(Self::Array),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolParameterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::Integer => write!(f, "integer"),
            Self::Number => write!(f, "number"),
            Self::Boolean => write!(f, "boolean"),
            Self::Array => write!(f, "array"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
