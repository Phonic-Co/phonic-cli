use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ConversationsClient {
    pub http_client: HttpClient,
}

impl ConversationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns conversations with optional filtering.
    ///
    /// # Arguments
    ///
    /// * `project` - The name of the project to list conversations for.
    /// * `external_id` - Filter by external ID to get a specific conversation.
    /// * `duration_min` - Minimum duration in seconds.
    /// * `duration_max` - Maximum duration in seconds.
    /// * `started_at_min` - Minimum start date/time. Valid examples: `2025-04-17`, `2025-04-17T02:48:52.708Z`
    /// * `started_at_max` - Maximum start date/time. Valid examples: `2025-04-17`, `2025-04-17T02:48:52.708Z`
    /// * `before` - Cursor for backward pagination. Use a conversation ID from `pagination.prev_cursor` to fetch the previous page of conversations. Cannot be used with `after`.
    /// * `after` - Cursor for forward pagination. Use a conversation ID from `pagination.next_cursor` to fetch the next page of conversations. Cannot be used with `before`.
    /// * `limit` - Maximum number of conversations to return per page.
    /// * `audio_container` - Format of the presigned `audio_url` in each conversation in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ConversationsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsListResponse, ApiError> {
        let base_url = self
            .http_client
            .config()
            .environment
            .as_ref()
            .map_or(self.http_client.base_url(), |env| env.base_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                "conversations",
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .string("external_id", request.external_id.clone())
                    .int("duration_min", request.duration_min.clone())
                    .int("duration_max", request.duration_max.clone())
                    .string("started_at_min", request.started_at_min.clone())
                    .string("started_at_max", request.started_at_max.clone())
                    .string("before", request.before.clone())
                    .string("after", request.after.clone())
                    .int("limit", request.limit.clone())
                    .serialize("audio_container", request.audio_container.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns a conversation by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the conversation to get.
    /// * `audio_container` - Format of the presigned `audio_url` in the response.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        id: &str,
        request: &ConversationsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsGetResponse, ApiError> {
        let base_url = self
            .http_client
            .config()
            .environment
            .as_ref()
            .map_or(self.http_client.base_url(), |env| env.base_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                &format!("conversations/{}", id),
                None,
                QueryBuilder::new()
                    .serialize("audio_container", request.audio_container.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes a conversation, scheduling its transcripts and audio recordings for deletion. The conversation must have ended.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the conversation to delete.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsDeleteResponse, ApiError> {
        let base_url = self
            .http_client
            .config()
            .environment
            .as_ref()
            .map_or(self.http_client.base_url(), |env| env.base_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::DELETE,
                &format!("conversations/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Cancels an active conversation.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the conversation to cancel.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn cancel(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsCancelResponse, ApiError> {
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
                &format!("conversations/{}/cancel", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns an analysis of the specified conversation.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the conversation to analyze.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_analysis(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsGetAnalysisResponse, ApiError> {
        let base_url = self
            .http_client
            .config()
            .environment
            .as_ref()
            .map_or(self.http_client.base_url(), |env| env.base_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                &format!("conversations/{}/analysis", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Returns all extractions for a conversation.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the conversation to get extractions for.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_extractions(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsListExtractionsResponse, ApiError> {
        let base_url = self
            .http_client
            .config()
            .environment
            .as_ref()
            .map_or(self.http_client.base_url(), |env| env.base_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                &format!("conversations/{}/extractions", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Extracts data from a conversation using a schema.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the conversation to extract data from.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn extract_data(
        &self,
        id: &str,
        request: &ExtractDataRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsExtractDataResponse, ApiError> {
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
                &format!("conversations/{}/extractions", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns all evaluations for a conversation.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the conversation to get evaluations for.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_evaluations(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsListEvaluationsResponse, ApiError> {
        let base_url = self
            .http_client
            .config()
            .environment
            .as_ref()
            .map_or(self.http_client.base_url(), |env| env.base_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::GET,
                &format!("conversations/{}/evals", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Evaluates a conversation using an evaluation prompt.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the conversation to evaluate.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn evaluate(
        &self,
        id: &str,
        request: &EvaluateConversationRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsEvaluateResponse, ApiError> {
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
                &format!("conversations/{}/evals", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Initiates a call to a given phone number using Phonic's Twilio account.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn outbound_call(
        &self,
        request: &OutboundCallRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsOutboundCallResponse, ApiError> {
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
                "conversations/outbound_call",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Initiates a SIP outbound call using user-supplied SIP credentials in headers.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn sip_outbound_call(
        &self,
        request: &ConversationsSipOutboundCallRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsSipOutboundCallResponse, ApiError> {
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
                "conversations/sip/outbound_call",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Replays an ended conversation by re-running its recorded audio through an
    /// agent. Requires API key or access token authentication. The conversation must
    /// have audio recordings available and an associated agent (or one specified in
    /// the request body).
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the conversation to replay.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn replay(
        &self,
        id: &str,
        request: &ReplayConversationRequest,
        options: Option<RequestOptions>,
    ) -> Result<ConversationsReplayResponse, ApiError> {
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
                &format!("conversations/{}/replay", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
