pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksListResponse {
    #[serde(default)]
    pub webhooks: Vec<Webhook>,
}

impl WebhooksListResponse {
    pub fn builder() -> WebhooksListResponseBuilder {
        <WebhooksListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksListResponseBuilder {
    webhooks: Option<Vec<Webhook>>,
}

impl WebhooksListResponseBuilder {
    pub fn webhooks(mut self, value: Vec<Webhook>) -> Self {
        self.webhooks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`webhooks`](WebhooksListResponseBuilder::webhooks)
    pub fn build(self) -> Result<WebhooksListResponse, BuildError> {
        Ok(WebhooksListResponse {
            webhooks: self.webhooks.ok_or_else(|| BuildError::missing_field("webhooks"))?,
        })
    }
}
