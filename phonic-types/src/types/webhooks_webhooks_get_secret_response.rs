pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksGetSecretResponse {
    /// The webhook's signing secret.
    #[serde(default)]
    pub signing_secret: String,
}

impl WebhooksGetSecretResponse {
    pub fn builder() -> WebhooksGetSecretResponseBuilder {
        <WebhooksGetSecretResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksGetSecretResponseBuilder {
    signing_secret: Option<String>,
}

impl WebhooksGetSecretResponseBuilder {
    pub fn signing_secret(mut self, value: impl Into<String>) -> Self {
        self.signing_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhooksGetSecretResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`signing_secret`](WebhooksGetSecretResponseBuilder::signing_secret)
    pub fn build(self) -> Result<WebhooksGetSecretResponse, BuildError> {
        Ok(WebhooksGetSecretResponse {
            signing_secret: self.signing_secret.ok_or_else(|| BuildError::missing_field("signing_secret"))?,
        })
    }
}
