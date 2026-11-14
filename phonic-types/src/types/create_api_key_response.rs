pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateApiKeyResponse {
    /// The API key ID.
    #[serde(default)]
    pub id: String,
    /// The name of the API key.
    #[serde(default)]
    pub name: String,
    /// The API key secret. Returned only in this response.
    #[serde(default)]
    pub api_key: String,
    /// The scope stored on the new key.
    pub scope: ApiKeyScope,
}

impl CreateApiKeyResponse {
    pub fn builder() -> CreateApiKeyResponseBuilder {
        <CreateApiKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApiKeyResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    api_key: Option<String>,
    scope: Option<ApiKeyScope>,
}

impl CreateApiKeyResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn scope(mut self, value: ApiKeyScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateApiKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateApiKeyResponseBuilder::id)
    /// - [`name`](CreateApiKeyResponseBuilder::name)
    /// - [`api_key`](CreateApiKeyResponseBuilder::api_key)
    /// - [`scope`](CreateApiKeyResponseBuilder::scope)
    pub fn build(self) -> Result<CreateApiKeyResponse, BuildError> {
        Ok(CreateApiKeyResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            api_key: self.api_key.ok_or_else(|| BuildError::missing_field("api_key"))?,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
        })
    }
}
