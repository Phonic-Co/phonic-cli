pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserStartedSpeakingPayload {
    pub r#type: String,
}

impl UserStartedSpeakingPayload {
    pub fn builder() -> UserStartedSpeakingPayloadBuilder {
        <UserStartedSpeakingPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserStartedSpeakingPayloadBuilder {
    r#type: Option<String>,
}

impl UserStartedSpeakingPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserStartedSpeakingPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](UserStartedSpeakingPayloadBuilder::r#type)
    pub fn build(self) -> Result<UserStartedSpeakingPayload, BuildError> {
        Ok(UserStartedSpeakingPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
