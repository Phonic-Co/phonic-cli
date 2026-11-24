pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksGetResponse {
    #[serde(default)]
    pub webhook: Webhook,
}

impl WebhooksGetResponse {
    pub fn builder() -> WebhooksGetResponseBuilder {
        <WebhooksGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksGetResponseBuilder {
    webhook: Option<Webhook>,
}

impl WebhooksGetResponseBuilder {
    pub fn webhook(mut self, value: Webhook) -> Self {
        self.webhook = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhook`](WebhooksGetResponseBuilder::webhook)
    pub fn build(self) -> Result<WebhooksGetResponse, BuildError> {
        Ok(WebhooksGetResponse {
            webhook: self.webhook.ok_or_else(|| BuildError::missing_field("webhook"))?,
        })
    }
}
