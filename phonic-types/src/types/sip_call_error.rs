pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct SipCallError {
    pub error: SipCallErrorError,
}

impl SipCallError {
    pub fn builder() -> SipCallErrorBuilder {
        <SipCallErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SipCallErrorBuilder {
    error: Option<SipCallErrorError>,
}

impl SipCallErrorBuilder {
    pub fn error(mut self, value: SipCallErrorError) -> Self {
        self.error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SipCallError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`error`](SipCallErrorBuilder::error)
    pub fn build(self) -> Result<SipCallError, BuildError> {
        Ok(SipCallError {
            error: self.error.ok_or_else(|| BuildError::missing_field("error"))?,
        })
    }
}
