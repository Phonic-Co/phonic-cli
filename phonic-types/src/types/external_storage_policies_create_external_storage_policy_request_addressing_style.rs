pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// How bucket names are addressed in requests to the endpoint.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateExternalStoragePolicyRequestAddressingStyle {
    Auto,
    Virtual,
    Path,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateExternalStoragePolicyRequestAddressingStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Auto => serializer.serialize_str("auto"),
            Self::Virtual => serializer.serialize_str("virtual"),
            Self::Path => serializer.serialize_str("path"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateExternalStoragePolicyRequestAddressingStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "auto" => Ok(Self::Auto),
            "virtual" => Ok(Self::Virtual),
            "path" => Ok(Self::Path),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateExternalStoragePolicyRequestAddressingStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Virtual => write!(f, "virtual"),
            Self::Path => write!(f, "path"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
