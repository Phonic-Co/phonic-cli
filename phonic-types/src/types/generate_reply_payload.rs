pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct GenerateReplyPayload {
    pub r#type: String,
    /// Optional system message to guide the assistant's reply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_message: Option<String>,
}

impl GenerateReplyPayload {
    pub fn builder() -> GenerateReplyPayloadBuilder {
        <GenerateReplyPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GenerateReplyPayloadBuilder {
    r#type: Option<String>,
    system_message: Option<String>,
}

impl GenerateReplyPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn system_message(mut self, value: impl Into<String>) -> Self {
        self.system_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GenerateReplyPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](GenerateReplyPayloadBuilder::r#type)
    pub fn build(self) -> Result<GenerateReplyPayload, BuildError> {
        Ok(GenerateReplyPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            system_message: self.system_message,
        })
    }
}
