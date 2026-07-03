pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssistantFinishedSpeakingPayload {
    pub r#type: String,
}

impl AssistantFinishedSpeakingPayload {
    pub fn builder() -> AssistantFinishedSpeakingPayloadBuilder {
        <AssistantFinishedSpeakingPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssistantFinishedSpeakingPayloadBuilder {
    r#type: Option<String>,
}

impl AssistantFinishedSpeakingPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssistantFinishedSpeakingPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AssistantFinishedSpeakingPayloadBuilder::r#type)
    pub fn build(self) -> Result<AssistantFinishedSpeakingPayload, BuildError> {
        Ok(AssistantFinishedSpeakingPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
