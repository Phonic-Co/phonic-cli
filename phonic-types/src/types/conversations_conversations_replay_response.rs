pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsReplayResponse {
    /// Confirmation that the replay has started.
    #[serde(default)]
    pub message: String,
}

impl ConversationsReplayResponse {
    pub fn builder() -> ConversationsReplayResponseBuilder {
        <ConversationsReplayResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsReplayResponseBuilder {
    message: Option<String>,
}

impl ConversationsReplayResponseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationsReplayResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](ConversationsReplayResponseBuilder::message)
    pub fn build(self) -> Result<ConversationsReplayResponse, BuildError> {
        Ok(ConversationsReplayResponse {
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
