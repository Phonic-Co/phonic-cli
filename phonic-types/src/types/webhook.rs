pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Webhook {
    /// The webhook ID.
    #[serde(default)]
    pub id: String,
    /// The URL that receives the webhook's requests.
    #[serde(default)]
    pub endpoint_url: String,
    /// The events sent to the endpoint.
    #[serde(default)]
    pub events: Vec<WebhookEvent>,
    /// The projects whose events are sent to the endpoint.
    #[serde(default)]
    pub project_ids: Vec<String>,
    /// When the webhook was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
}

impl Webhook {
    pub fn builder() -> WebhookBuilder {
        <WebhookBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookBuilder {
    id: Option<String>,
    endpoint_url: Option<String>,
    events: Option<Vec<WebhookEvent>>,
    project_ids: Option<Vec<String>>,
    created_at: Option<DateTime<FixedOffset>>,
}

impl WebhookBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

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

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Webhook`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WebhookBuilder::id)
    /// - [`endpoint_url`](WebhookBuilder::endpoint_url)
    /// - [`events`](WebhookBuilder::events)
    /// - [`project_ids`](WebhookBuilder::project_ids)
    /// - [`created_at`](WebhookBuilder::created_at)
    pub fn build(self) -> Result<Webhook, BuildError> {
        Ok(Webhook {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            endpoint_url: self.endpoint_url.ok_or_else(|| BuildError::missing_field("endpoint_url"))?,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
            project_ids: self.project_ids.ok_or_else(|| BuildError::missing_field("project_ids"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
