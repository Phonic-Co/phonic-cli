use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ToolsClient {
    pub http_client: HttpClient,
}

impl ToolsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns all custom tools for the organization.
    ///
    /// # Arguments
    ///
    /// * `project` - The name of the project to list tools for.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ToolsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ToolsListResponse, ApiError> {
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
                "tools",
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new tool in a project.
    ///
    /// # Arguments
    ///
    /// * `project` - The name of the project to create the tool in.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &CreateToolRequest,
        options: Option<RequestOptions>,
    ) -> Result<ToolsCreateResponse, ApiError> {
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
                "tools",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns a tool by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the tool to get.
    /// * `project` - The name of the project containing the tool. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        name_or_id: &str,
        request: &ToolsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ToolsGetResponse, ApiError> {
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
                &format!("tools/{}", name_or_id),
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes a tool by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the tool to delete.
    /// * `project` - The name of the project containing the tool. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        name_or_id: &str,
        request: &ToolsDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ToolsDeleteResponse, ApiError> {
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
                &format!("tools/{}", name_or_id),
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates a tool by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the tool to update.
    /// * `project` - The name of the project containing the tool. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        name_or_id: &str,
        request: &UpdateToolRequest,
        options: Option<RequestOptions>,
    ) -> Result<ToolsUpdateResponse, ApiError> {
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
                &format!("tools/{}", name_or_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }
}
