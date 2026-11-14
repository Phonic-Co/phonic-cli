pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWebhookResponse {
    /// The webhook ID.
    #[serde(default)]
    pub id: String,
    /// The secret used to sign the webhook's requests. It can be fetched again from `GET /webhooks/{id}/secret`.
    #[serde(default)]
    pub signing_secret: String,
}

impl CreateWebhookResponse {
    pub fn builder() -> CreateWebhookResponseBuilder {
        <CreateWebhookResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhookResponseBuilder {
    id: Option<String>,
    signing_secret: Option<String>,
}

impl CreateWebhookResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn signing_secret(mut self, value: impl Into<String>) -> Self {
        self.signing_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWebhookResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateWebhookResponseBuilder::id)
    /// - [`signing_secret`](CreateWebhookResponseBuilder::signing_secret)
    pub fn build(self) -> Result<CreateWebhookResponse, BuildError> {
        Ok(CreateWebhookResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            signing_secret: self.signing_secret.ok_or_else(|| BuildError::missing_field("signing_secret"))?,
        })
    }
}
