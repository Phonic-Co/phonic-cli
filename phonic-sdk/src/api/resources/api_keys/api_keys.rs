use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ApiKeysClient {
    pub http_client: HttpClient,
}

impl ApiKeysClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a new API key in the workspace.
    ///
    /// Requires an API key with workspace access, or an access token. The new
    /// key's `scope` must be within the caller's scope. Omit `scope` and an
    /// API key copies its own scope onto the new key. An access token that
    /// omits `scope` creates a key with access to every project and to
    /// workspace management.
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
        request: &CreateApiKeyRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreateApiKeyResponse, ApiError> {
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
                "api_keys",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes an API key.
    ///
    /// Requires an API key with workspace access, or an access token. The
    /// caller can delete a key only when that key's scope is within the
    /// caller's scope.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the API key to delete.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ApiKeysDeleteResponse, ApiError> {
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
                &format!("api_keys/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates an API key's name, scope, or both. Send at least one of `name`
    /// and `scope`.
    ///
    /// Requires an API key with workspace access, or an access token. The
    /// caller can update a key only when that key's current scope is within
    /// the caller's scope, and a new `scope` must also be within the caller's
    /// scope.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the API key to update.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        id: &str,
        request: &UpdateApiKeyRequest,
        options: Option<RequestOptions>,
    ) -> Result<ApiKeysUpdateResponse, ApiError> {
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
                &format!("api_keys/{}", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Rotates an API key, generating a new secret and invalidating the old
    /// one. The new key keeps the previous key's name and scope.
    ///
    /// Requires an API key with workspace access, or an access token. The
    /// caller can rotate a key only when that key's scope is within the
    /// caller's scope.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the API key to rotate.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn rotate(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ApiKeyWithSecret, ApiError> {
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
                &format!("api_keys/{}/rotate", id),
                None,
                None,
                options,
            )
            .await
    }
}
