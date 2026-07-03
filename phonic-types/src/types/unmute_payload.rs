pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UnmutePayload {
    pub r#type: String,
}

impl UnmutePayload {
    pub fn builder() -> UnmutePayloadBuilder {
        <UnmutePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnmutePayloadBuilder {
    r#type: Option<String>,
}

impl UnmutePayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UnmutePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](UnmutePayloadBuilder::r#type)
    pub fn build(self) -> Result<UnmutePayload, BuildError> {
        Ok(UnmutePayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
