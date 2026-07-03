use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ConversationItemsClient {
    pub http_client: HttpClient,
}

impl ConversationItemsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the alternative response(s) the assistant would have
    /// produced for this conversation turn given changes to the agent system prompt.
    ///
    /// Only assistant items from ended conversations can be replayed. The
    /// conversation must have an associated agent.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the conversation item to replay.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn replay(
        &self,
        id: &str,
        request: &ReplayConversationItemRequest,
        options: Option<RequestOptions>,
    ) -> Result<ReplayConversationItemResponse, ApiError> {
        let base_url = self
            .http_client
            .config()
            .environment
            .as_ref()
            .map_or(self.http_client.base_url(), |env| env.base_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::POST,
                &format!("conversation_items/{}/replay", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
