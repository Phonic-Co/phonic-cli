use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ExtractionSchemasClient {
    pub http_client: HttpClient,
}

impl ExtractionSchemasClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns all extraction schemas in a project.
    ///
    /// # Arguments
    ///
    /// * `project` - The name of the project to list extraction schemas for.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ExtractionSchemasListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExtractionSchemasListResponse, ApiError> {
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
                "extraction_schemas",
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new extraction schema in a project.
    ///
    /// # Arguments
    ///
    /// * `project` - The name of the project to create the extraction schema in.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &CreateExtractionSchemaRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExtractionSchemasCreateResponse, ApiError> {
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
                "extraction_schemas",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns an extraction schema by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the extraction schema to get.
    /// * `project` - The name of the project containing the extraction schema. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        name_or_id: &str,
        request: &ExtractionSchemasGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExtractionSchemasGetResponse, ApiError> {
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
                &format!("extraction_schemas/{}", name_or_id),
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes an extraction schema by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the extraction schema to delete.
    /// * `project` - The name of the project containing the extraction schema. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        name_or_id: &str,
        request: &ExtractionSchemasDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExtractionSchemasDeleteResponse, ApiError> {
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
                &format!("extraction_schemas/{}", name_or_id),
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates an extraction schema by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the extraction schema to update.
    /// * `project` - The name of the project containing the extraction schema. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        name_or_id: &str,
        request: &UpdateExtractionSchemaRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExtractionSchemasUpdateResponse, ApiError> {
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
                &format!("extraction_schemas/{}", name_or_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }
}
