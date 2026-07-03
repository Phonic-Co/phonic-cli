pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsGetQueryRequest {
    /// Format of the presigned `audio_url` in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_container: Option<ConversationsGetRequestAudioContainer>,
}

impl ConversationsGetQueryRequest {
    pub fn builder() -> ConversationsGetQueryRequestBuilder {
        <ConversationsGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsGetQueryRequestBuilder {
    audio_container: Option<ConversationsGetRequestAudioContainer>,
}

impl ConversationsGetQueryRequestBuilder {
    pub fn audio_container(mut self, value: ConversationsGetRequestAudioContainer) -> Self {
        self.audio_container = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsGetQueryRequest`].
    pub fn build(self) -> Result<ConversationsGetQueryRequest, BuildError> {
        Ok(ConversationsGetQueryRequest {
            audio_container: self.audio_container,
        })
    }
}

