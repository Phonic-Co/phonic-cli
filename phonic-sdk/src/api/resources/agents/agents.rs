use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AgentsClient {
    pub http_client: HttpClient,
}

impl AgentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns all agents in a project.
    ///
    /// # Arguments
    ///
    /// * `project` - The name of the project to list agents for.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &AgentsListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentsListResponse, ApiError> {
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
                "agents",
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a new agent in a project.
    ///
    /// # Arguments
    ///
    /// * `project` - The name of the project to create the agent in.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &CreateRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentsCreateResponse, ApiError> {
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
                "agents",
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Upserts an agent by name. If an agent with the same name already exists, it will be updated. Otherwise, it will be created.
    ///
    /// # Arguments
    ///
    /// * `project` - The name of the project containing the agent.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn upsert(
        &self,
        request: &UpsertAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentsUpsertResponse, ApiError> {
        let base_url = self
            .http_client
            .config()
            .environment
            .as_ref()
            .map_or(self.http_client.base_url(), |env| env.base_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::PUT,
                "agents/upsert",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns an agent by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the agent to get.
    /// * `project` - The name of the project containing the agent. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        name_or_id: &str,
        request: &AgentsGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentsGetResponse, ApiError> {
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
                &format!("agents/{}", name_or_id),
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes an agent by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the agent to delete.
    /// * `project` - The name of the project containing the agent. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        name_or_id: &str,
        request: &AgentsDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentsDeleteResponse, ApiError> {
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
                &format!("agents/{}", name_or_id),
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates an agent by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the agent to update.
    /// * `project` - The name of the project containing the agent. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        name_or_id: &str,
        request: &UpdateAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentsUpdateResponse, ApiError> {
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
                &format!("agents/{}", name_or_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Adds a custom phone number to an agent. The user must configure their SIP trunk to point to Phonic's SIP server.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the agent.
    /// * `project` - The name of the project containing the agent. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_custom_phone_number(
        &self,
        name_or_id: &str,
        request: &AgentsAddCustomPhoneNumberRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentsAddCustomPhoneNumberResponse, ApiError> {
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
                &format!("agents/{}/custom-phone-numbers", name_or_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes a custom phone number from an agent.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the agent.
    /// * `project` - The name of the project containing the agent. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_custom_phone_number(
        &self,
        name_or_id: &str,
        request: &AgentsDeleteCustomPhoneNumberRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentsDeleteCustomPhoneNumberResponse, ApiError> {
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
                &format!("agents/{}/custom-phone-numbers", name_or_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates a phone number on an agent.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the agent.
    /// * `project` - The name of the project containing the agent. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_phone_number(
        &self,
        name_or_id: &str,
        request: &AgentsUpdatePhoneNumberRequest,
        options: Option<RequestOptions>,
    ) -> Result<AgentsUpdatePhoneNumberResponse, ApiError> {
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
                &format!("agents/{}/phone-numbers", name_or_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }
}
