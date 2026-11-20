use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct WebhooksClient {
    pub http_client: HttpClient,
}

impl WebhooksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the workspace's webhooks, newest first.
    ///
    /// Requires workspace access. An API key restricted to selected projects
    /// only sees webhooks whose projects are all within its scope.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksListResponse, ApiError> {
        let base_url = self
            .http_client
            .config()
            .environment
            .as_ref()
            .map_or(self.http_client.base_url(), |env| env.base_url());
        self.http_client
            .execute_request_with_base_url(base_url, Method::GET, "webhooks", None, None, options)
            .await
    }

    /// Creates a webhook that sends the chosen events for the chosen projects
    /// to `endpoint_url`. The response includes the webhook's signing secret.
    ///
    /// Requires an API key with workspace access, or an access token. An API
    /// key restricted to selected projects can only use projects within its
    /// scope.
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
        request: &CreateWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateWebhookResponse, ApiError> {
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
                "webhooks",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a webhook.
    ///
    /// Requires workspace access. An API key restricted to selected projects
    /// can only get a webhook whose projects are all within its scope.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the webhook.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksGetResponse, ApiError> {
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
                &format!("webhooks/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a webhook.
    ///
    /// Requires an API key with workspace access, or an access token. An API
    /// key restricted to selected projects can only delete a webhook whose
    /// projects are all within its scope.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the webhook to delete.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksDeleteResponse, ApiError> {
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
                &format!("webhooks/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a webhook's events, projects, or both. The endpoint URL cannot
    /// be changed; create a new webhook instead.
    ///
    /// Requires an API key with workspace access, or an access token. An API
    /// key restricted to selected projects can only update a webhook whose
    /// current and new projects are all within its scope.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the webhook to update.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        id: &str,
        request: &UpdateWebhookRequest,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksUpdateResponse, ApiError> {
        let base_url = self
            .http_client
            .config()
            .environment
            .as_ref()
            .map_or(self.http_client.base_url(), |env| env.base_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::PATCH,
                &format!("webhooks/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns the secret used to sign the webhook's requests.
    ///
    /// Requires an API key with workspace access, or an access token. An API
    /// key restricted to selected projects can only get the secret of a
    /// webhook whose projects are all within its scope.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the webhook.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_secret(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WebhooksGetSecretResponse, ApiError> {
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
                &format!("webhooks/{}/secret", id),
                None,
                None,
                options,
            )
            .await
    }
}
