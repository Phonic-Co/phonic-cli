pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWebhookRequest {
    /// The URL that receives the webhook's requests. Must be a publicly
    /// routable HTTPS URL without embedded credentials, and unique within
    /// the workspace. It cannot be changed later.
    #[serde(default)]
    pub endpoint_url: String,
    /// The events to send to the endpoint.
    #[serde(default)]
    pub events: Vec<WebhookEvent>,
    /// The projects whose events are sent to the endpoint.
    #[serde(default)]
    pub project_ids: Vec<String>,
}

impl CreateWebhookRequest {
    pub fn builder() -> CreateWebhookRequestBuilder {
        <CreateWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWebhookRequestBuilder {
    endpoint_url: Option<String>,
    events: Option<Vec<WebhookEvent>>,
    project_ids: Option<Vec<String>>,
}

impl CreateWebhookRequestBuilder {
    pub fn endpoint_url(mut self, value: impl Into<String>) -> Self {
        self.endpoint_url = Some(value.into());
        self
    }

    pub fn events(mut self, value: Vec<WebhookEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn project_ids(mut self, value: Vec<String>) -> Self {
        self.project_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateWebhookRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`endpoint_url`](CreateWebhookRequestBuilder::endpoint_url)
    /// - [`events`](CreateWebhookRequestBuilder::events)
    /// - [`project_ids`](CreateWebhookRequestBuilder::project_ids)
    pub fn build(self) -> Result<CreateWebhookRequest, BuildError> {
        Ok(CreateWebhookRequest {
            endpoint_url: self.endpoint_url.ok_or_else(|| BuildError::missing_field("endpoint_url"))?,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
            project_ids: self.project_ids.ok_or_else(|| BuildError::missing_field("project_ids"))?,
        })
    }
}

