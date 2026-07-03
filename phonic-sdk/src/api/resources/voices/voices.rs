use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct VoicesClient {
    pub http_client: HttpClient,
}

impl VoicesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns all available voices for a model.
    ///
    /// # Arguments
    ///
    /// * `model` - The model to get voices for.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &VoicesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<VoicesListResponse, ApiError> {
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
                "voices",
                None,
                QueryBuilder::new()
                    .string("model", request.model.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns a voice by ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the voice to get.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<VoicesGetResponse, ApiError> {
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
                &format!("voices/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
