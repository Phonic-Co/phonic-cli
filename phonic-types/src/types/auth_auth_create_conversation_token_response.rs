pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthCreateConversationTokenResponse {
    /// The conversation token to use for authentication.
    #[serde(default)]
    pub conversation_token: String,
    /// When the conversation token expires.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
}

impl AuthCreateConversationTokenResponse {
    pub fn builder() -> AuthCreateConversationTokenResponseBuilder {
        <AuthCreateConversationTokenResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthCreateConversationTokenResponseBuilder {
    conversation_token: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl AuthCreateConversationTokenResponseBuilder {
    pub fn conversation_token(mut self, value: impl Into<String>) -> Self {
        self.conversation_token = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthCreateConversationTokenResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conversation_token`](AuthCreateConversationTokenResponseBuilder::conversation_token)
    /// - [`expires_at`](AuthCreateConversationTokenResponseBuilder::expires_at)
    pub fn build(self) -> Result<AuthCreateConversationTokenResponse, BuildError> {
        Ok(AuthCreateConversationTokenResponse {
            conversation_token: self.conversation_token.ok_or_else(|| BuildError::missing_field("conversation_token"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
