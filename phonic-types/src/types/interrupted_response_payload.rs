pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InterruptedResponsePayload {
    pub r#type: String,
    /// The part of the assistant's turn that was spoken before the user interrupted.
    #[serde(default)]
    pub text: String,
}

impl InterruptedResponsePayload {
    pub fn builder() -> InterruptedResponsePayloadBuilder {
        <InterruptedResponsePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InterruptedResponsePayloadBuilder {
    r#type: Option<String>,
    text: Option<String>,
}

impl InterruptedResponsePayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InterruptedResponsePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](InterruptedResponsePayloadBuilder::r#type)
    /// - [`text`](InterruptedResponsePayloadBuilder::text)
    pub fn build(self) -> Result<InterruptedResponsePayload, BuildError> {
        Ok(InterruptedResponsePayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
