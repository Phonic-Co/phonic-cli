pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The field type.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExtractionFieldType {
    String,
    Int,
    Float,
    Bool,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ExtractionFieldType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::String => serializer.serialize_str("string"),
            Self::Int => serializer.serialize_str("int"),
            Self::Float => serializer.serialize_str("float"),
            Self::Bool => serializer.serialize_str("bool"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ExtractionFieldType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "string" => Ok(Self::String),
            "int" => Ok(Self::Int),
            "float" => Ok(Self::Float),
            "bool" => Ok(Self::Bool),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ExtractionFieldType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::Int => write!(f, "int"),
            Self::Float => write!(f, "float"),
            Self::Bool => write!(f, "bool"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
