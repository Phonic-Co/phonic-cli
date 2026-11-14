use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct McpServersClient {
    pub http_client: HttpClient,
}

impl McpServersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns all MCP servers in a project.
    ///
    /// # Arguments
    ///
    /// * `project` - The name of the project to list MCP servers for.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &McpServersListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<McpServersListResponse, ApiError> {
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
                "mcp_servers",
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates an MCP server in the authenticated workspace and project. Phonic connects to the server, lists its tools and saves them alongside the server, so the server must be reachable from Phonic's network. The supplied headers are sent on every request Phonic makes to the server; their values are stored encrypted and never returned. Creating the server does not attach it to an agent; add its ID to an agent's `mcp_server_ids` to make its tools available.
    ///
    /// # Arguments
    ///
    /// * `project` - The name of the project to create the MCP server in.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &CreateMcpServerRequest,
        options: Option<RequestOptions>,
    ) -> Result<McpServersCreateResponse, ApiError> {
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
                "mcp_servers",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns an MCP server by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the MCP server to get.
    /// * `project` - The name of the project containing the MCP server. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        name_or_id: &str,
        request: &McpServersGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<McpServersGetResponse, ApiError> {
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
                &format!("mcp_servers/{}", name_or_id),
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes an MCP server by name or ID and removes it from every agent's `mcp_server_ids`.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the MCP server to delete.
    /// * `project` - The name of the project containing the MCP server. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        name_or_id: &str,
        request: &McpServersDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<McpServersDeleteResponse, ApiError> {
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
                &format!("mcp_servers/{}", name_or_id),
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates the name and/or headers of an existing MCP server by name or ID. Providing `headers` replaces all existing headers. The URL cannot be changed; delete and re-create the server instead.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the MCP server to update.
    /// * `project` - The name of the project containing the MCP server. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        name_or_id: &str,
        request: &UpdateMcpServerRequest,
        options: Option<RequestOptions>,
    ) -> Result<McpServersUpdateResponse, ApiError> {
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
                &format!("mcp_servers/{}", name_or_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }
}
