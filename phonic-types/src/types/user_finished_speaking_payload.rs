pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserFinishedSpeakingPayload {
    pub r#type: String,
}

impl UserFinishedSpeakingPayload {
    pub fn builder() -> UserFinishedSpeakingPayloadBuilder {
        <UserFinishedSpeakingPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserFinishedSpeakingPayloadBuilder {
    r#type: Option<String>,
}

impl UserFinishedSpeakingPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserFinishedSpeakingPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](UserFinishedSpeakingPayloadBuilder::r#type)
    pub fn build(self) -> Result<UserFinishedSpeakingPayload, BuildError> {
        Ok(UserFinishedSpeakingPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
