pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateToolRequestParameterLocationsValue {
    RequestBody,
    QueryString,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateToolRequestParameterLocationsValue {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RequestBody => serializer.serialize_str("request_body"),
            Self::QueryString => serializer.serialize_str("query_string"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateToolRequestParameterLocationsValue {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "request_body" => Ok(Self::RequestBody),
            "query_string" => Ok(Self::QueryString),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateToolRequestParameterLocationsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestBody => write!(f, "request_body"),
            Self::QueryString => write!(f, "query_string"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
