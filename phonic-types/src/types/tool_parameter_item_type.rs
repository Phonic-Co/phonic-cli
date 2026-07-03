pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Required only when type is "array". The type of items in the array.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolParameterItemType {
    String,
    Integer,
    Number,
    Boolean,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolParameterItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::String => serializer.serialize_str("string"),
            Self::Integer => serializer.serialize_str("integer"),
            Self::Number => serializer.serialize_str("number"),
            Self::Boolean => serializer.serialize_str("boolean"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolParameterItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "string" => Ok(Self::String),
            "integer" => Ok(Self::Integer),
            "number" => Ok(Self::Number),
            "boolean" => Ok(Self::Boolean),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolParameterItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::Integer => write!(f, "integer"),
            Self::Number => write!(f, "number"),
            Self::Boolean => write!(f, "boolean"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
