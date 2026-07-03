pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssistantChoseNotToRespondPayload {
    pub r#type: String,
}

impl AssistantChoseNotToRespondPayload {
    pub fn builder() -> AssistantChoseNotToRespondPayloadBuilder {
        <AssistantChoseNotToRespondPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssistantChoseNotToRespondPayloadBuilder {
    r#type: Option<String>,
}

impl AssistantChoseNotToRespondPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssistantChoseNotToRespondPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AssistantChoseNotToRespondPayloadBuilder::r#type)
    pub fn build(self) -> Result<AssistantChoseNotToRespondPayload, BuildError> {
        Ok(AssistantChoseNotToRespondPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
