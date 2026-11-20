use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ExternalStoragePoliciesClient {
    pub http_client: HttpClient,
}

impl ExternalStoragePoliciesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns all external storage policies in a project.
    ///
    /// # Arguments
    ///
    /// * `project` - The name of the project to list external storage policies for.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list(
        &self,
        request: &ExternalStoragePoliciesListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExternalStoragePoliciesListResponse, ApiError> {
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
                "external_storage_policies",
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a named external storage policy in the authenticated workspace and project, storing the supplied S3-compatible destination and credentials for later artifact delivery. Use this when the user wants agents to export conversation artifacts to a bucket they are authorized to use. Agents referencing this policy send conversation artifacts, which may contain personal data, to that external destination. Creating the policy does not attach it to an agent or export existing conversations.
    ///
    /// # Arguments
    ///
    /// * `project` - The name of the project to create the external storage policy in.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &CreateExternalStoragePolicyRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExternalStoragePoliciesCreateResponse, ApiError> {
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
                "external_storage_policies",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns an external storage policy by name or ID.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the external storage policy to get.
    /// * `project` - The name of the project containing the external storage policy. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get(
        &self,
        name_or_id: &str,
        request: &ExternalStoragePoliciesGetQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExternalStoragePoliciesGetResponse, ApiError> {
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
                &format!("external_storage_policies/{}", name_or_id),
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Deletes an external storage policy by name or ID. The policy must not be referenced by any agent.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the external storage policy to delete.
    /// * `project` - The name of the project containing the external storage policy. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete(
        &self,
        name_or_id: &str,
        request: &ExternalStoragePoliciesDeleteQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExternalStoragePoliciesDeleteResponse, ApiError> {
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
                &format!("external_storage_policies/{}", name_or_id),
                None,
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates the supplied fields of an existing external storage policy by name or ID in the authenticated workspace and project. Use this to change the artifact destination or rotate its credentials. Changes affect subsequent artifact delivery for agents referencing the policy; changing the endpoint or bucket can send conversation artifacts, which may contain personal data, to a different external destination. Confirm the intended destination before updating. Rotate credentials by providing both `access_key_id` and `secret_access_key`. This operation does not move or delete artifacts already delivered.
    ///
    /// # Arguments
    ///
    /// * `name_or_id` - The name or the ID of the external storage policy to update.
    /// * `project` - The name of the project containing the external storage policy. Only used when `nameOrId` is a name.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update(
        &self,
        name_or_id: &str,
        request: &UpdateExternalStoragePolicyRequest,
        options: Option<RequestOptions>,
    ) -> Result<ExternalStoragePoliciesUpdateResponse, ApiError> {
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
                &format!("external_storage_policies/{}", name_or_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("project", request.project.clone())
                    .build(),
                options,
            )
            .await
    }
}
