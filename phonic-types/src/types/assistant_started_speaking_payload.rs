pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AssistantStartedSpeakingPayload {
    pub r#type: String,
}

impl AssistantStartedSpeakingPayload {
    pub fn builder() -> AssistantStartedSpeakingPayloadBuilder {
        <AssistantStartedSpeakingPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssistantStartedSpeakingPayloadBuilder {
    r#type: Option<String>,
}

impl AssistantStartedSpeakingPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssistantStartedSpeakingPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AssistantStartedSpeakingPayloadBuilder::r#type)
    pub fn build(self) -> Result<AssistantStartedSpeakingPayload, BuildError> {
        Ok(AssistantStartedSpeakingPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
