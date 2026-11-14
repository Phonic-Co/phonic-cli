pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSessionTokenRequest {
    /// Time-to-live for the session token in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<i64>,
    /// Restricts the token to these conversations. A restricted token can read only their live audio and transcript, and cannot open the STS WebSocket or create an STS session. Omit it and the token can read any live conversation in the org. Pass it whenever the token will reach an end user's browser.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_ids: Option<Vec<String>>,
}

impl CreateSessionTokenRequest {
    pub fn builder() -> CreateSessionTokenRequestBuilder {
        <CreateSessionTokenRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSessionTokenRequestBuilder {
    ttl_seconds: Option<i64>,
    conversation_ids: Option<Vec<String>>,
}

impl CreateSessionTokenRequestBuilder {
    pub fn ttl_seconds(mut self, value: i64) -> Self {
        self.ttl_seconds = Some(value);
        self
    }

    pub fn conversation_ids(mut self, value: Vec<String>) -> Self {
        self.conversation_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSessionTokenRequest`].
    pub fn build(self) -> Result<CreateSessionTokenRequest, BuildError> {
        Ok(CreateSessionTokenRequest {
            ttl_seconds: self.ttl_seconds,
            conversation_ids: self.conversation_ids,
        })
    }
}

