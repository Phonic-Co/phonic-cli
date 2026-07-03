pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Error {
        BasicError(BasicError),

        ValidationError(ValidationError),
}

impl Error {
    pub fn is_basic_error(&self) -> bool {
        matches!(self, Self::BasicError(_))
    }

    pub fn is_validation_error(&self) -> bool {
        matches!(self, Self::ValidationError(_))
    }


    pub fn as_basic_error(&self) -> Option<&BasicError> {
        match self {
                    Self::BasicError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_basic_error(self) -> Option<BasicError> {
        match self {
                    Self::BasicError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_validation_error(&self) -> Option<&ValidationError> {
        match self {
                    Self::ValidationError(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_validation_error(self) -> Option<ValidationError> {
        match self {
                    Self::ValidationError(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BasicError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ValidationError(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
