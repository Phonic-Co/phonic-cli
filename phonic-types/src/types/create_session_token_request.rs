pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSessionTokenRequest {
    /// Time-to-live for the session token in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<i64>,
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
}

impl CreateSessionTokenRequestBuilder {
    pub fn ttl_seconds(mut self, value: i64) -> Self {
        self.ttl_seconds = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSessionTokenRequest`].
    pub fn build(self) -> Result<CreateSessionTokenRequest, BuildError> {
        Ok(CreateSessionTokenRequest {
            ttl_seconds: self.ttl_seconds,
        })
    }
}

