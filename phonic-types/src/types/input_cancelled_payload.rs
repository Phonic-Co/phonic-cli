pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InputCancelledPayload {
    pub r#type: String,
}

impl InputCancelledPayload {
    pub fn builder() -> InputCancelledPayloadBuilder {
        <InputCancelledPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InputCancelledPayloadBuilder {
    r#type: Option<String>,
}

impl InputCancelledPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InputCancelledPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](InputCancelledPayloadBuilder::r#type)
    pub fn build(self) -> Result<InputCancelledPayload, BuildError> {
        Ok(InputCancelledPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
