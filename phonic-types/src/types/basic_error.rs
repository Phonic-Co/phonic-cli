pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct BasicError {
    pub error: Option<BasicErrorError>,
}

impl BasicError {
    pub fn builder() -> BasicErrorBuilder {
        <BasicErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BasicErrorBuilder {
    error: Option<BasicErrorError>,
}

impl BasicErrorBuilder {
    pub fn error(mut self, value: BasicErrorError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BasicError`].
    pub fn build(self) -> Result<BasicError, BuildError> {
        Ok(BasicError {
            error: self.error,
        })
    }
}
