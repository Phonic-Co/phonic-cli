pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SayPayload {
    pub r#type: String,
    /// The text for the assistant to say
    #[serde(default)]
    pub text: String,
    /// Optional configuration to make the turn interruptible or not interruptible
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interruptible: Option<bool>,
}

impl SayPayload {
    pub fn builder() -> SayPayloadBuilder {
        <SayPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SayPayloadBuilder {
    r#type: Option<String>,
    text: Option<String>,
    interruptible: Option<bool>,
}

impl SayPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn interruptible(mut self, value: bool) -> Self {
        self.interruptible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SayPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](SayPayloadBuilder::r#type)
    /// - [`text`](SayPayloadBuilder::text)
    pub fn build(self) -> Result<SayPayload, BuildError> {
        Ok(SayPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            interruptible: self.interruptible,
        })
    }
}
