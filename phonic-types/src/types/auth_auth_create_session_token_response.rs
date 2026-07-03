pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthCreateSessionTokenResponse {
    /// The session token to use for authentication.
    #[serde(default)]
    pub session_token: String,
    /// When the session token expires.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
}

impl AuthCreateSessionTokenResponse {
    pub fn builder() -> AuthCreateSessionTokenResponseBuilder {
        <AuthCreateSessionTokenResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthCreateSessionTokenResponseBuilder {
    session_token: Option<String>,
    expires_at: Option<DateTime<FixedOffset>>,
}

impl AuthCreateSessionTokenResponseBuilder {
    pub fn session_token(mut self, value: impl Into<String>) -> Self {
        self.session_token = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthCreateSessionTokenResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session_token`](AuthCreateSessionTokenResponseBuilder::session_token)
    /// - [`expires_at`](AuthCreateSessionTokenResponseBuilder::expires_at)
    pub fn build(self) -> Result<AuthCreateSessionTokenResponse, BuildError> {
        Ok(AuthCreateSessionTokenResponse {
            session_token: self.session_token.ok_or_else(|| BuildError::missing_field("session_token"))?,
            expires_at: self.expires_at.ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
