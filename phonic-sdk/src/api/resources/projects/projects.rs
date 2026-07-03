use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ProjectsClient {
    pub http_client: HttpClient,
}

impl ProjectsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns all projects in a workspace.
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
    ) -> Result<ProjectsListResponse, ApiError> {
        let base_url = self
            .http_client
            .config()
            .environment
            .as_ref()
            .map_or(self.http_client.base_url(), |env| env.base_url());
        self.http_client
            .execute_request_with_base_url(base_url, Method::GET, "projects", None, None, options)
            .await
    }

    /// Creates a new project in a workspace.
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
        request: &CreateProjectRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProjectsCreateResponse, ApiError> {
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
                "projects",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a project by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the project to get.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        name_or_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProjectsGetResponse, ApiError> {
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
                &format!("projects/{}", name_or_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Deletes a project by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the project to delete.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        name_or_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProjectsDeleteResponse, ApiError> {
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
                &format!("projects/{}", name_or_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a project by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the project to update.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        name_or_id: &str,
        request: &UpdateProjectRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProjectsUpdateResponse, ApiError> {
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
                &format!("projects/{}", name_or_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns all conversation evaluation prompts for a project.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the project.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_eval_prompts(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProjectsListEvalPromptsResponse, ApiError> {
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
                &format!("projects/{}/conversation_eval_prompts", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Creates a new conversation evaluation prompt for a project.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the project.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create_eval_prompt(
        &self,
        id: &str,
        request: &CreateConversationEvalPromptRequest,
        options: Option<RequestOptions>,
    ) -> Result<ProjectsCreateEvalPromptResponse, ApiError> {
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
                &format!("projects/{}/conversation_eval_prompts", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns all conversation evaluation results for a project.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the project.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_evals(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ProjectsListEvalsResponse, ApiError> {
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
                &format!("projects/{}/conversation_evals", id),
                None,
                None,
                options,
            )
            .await
    }
}
