pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWebhookRequest {
    /// Replaces the events sent to the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<WebhookEvent>>,
    /// Replaces the projects whose events are sent to the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
}

impl UpdateWebhookRequest {
    pub fn builder() -> UpdateWebhookRequestBuilder {
        <UpdateWebhookRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWebhookRequestBuilder {
    events: Option<Vec<WebhookEvent>>,
    project_ids: Option<Vec<String>>,
}

impl UpdateWebhookRequestBuilder {
    pub fn events(mut self, value: Vec<WebhookEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn project_ids(mut self, value: Vec<String>) -> Self {
        self.project_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateWebhookRequest`].
    pub fn build(self) -> Result<UpdateWebhookRequest, BuildError> {
        Ok(UpdateWebhookRequest {
            events: self.events,
            project_ids: self.project_ids,
        })
    }
}

