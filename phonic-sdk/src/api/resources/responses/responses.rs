use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ResponsesClient {
    pub http_client: HttpClient,
}

impl ResponsesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Generates one or more alternative assistant responses for a conversation
    /// you supply inline to simulate Phonic agent behavior.
    ///
    /// This endpoint is stateless, so it does not create a new conversation or
    /// store anything. The request carries the system prompt, a conversation so
    /// far as `input`, and the tools the assistant may call as
    /// `tool_definitions`.
    ///
    /// Each item in `input` is a user message, an assistant message (with
    /// optional `tool_calls`), or a `tool_call_output`. Every assistant tool
    /// call must be followed immediately by the `tool_call_output` item that
    /// carries its result.
    ///
    /// This is an experimental feature and must be enabled for your workspace;
    /// otherwise, it returns `404`. Please contact our team if you would like
    /// access.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &GenerateResponsesRequest,
        options: Option<RequestOptions>,
    ) -> Result<GenerateResponsesResponse, ApiError> {
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
                "responses",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
