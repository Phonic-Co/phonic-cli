pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateApiKeyRequest {
    /// A name to identify the API key. Required, and unique within the workspace.
    #[serde(default)]
    pub name: String,
    /// Access granted to the new key. Must be within the caller's own
    /// scope. Omit it and an API key copies its own scope onto the new
    /// key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ApiKeyScope>,
}

impl CreateApiKeyRequest {
    pub fn builder() -> CreateApiKeyRequestBuilder {
        <CreateApiKeyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApiKeyRequestBuilder {
    name: Option<String>,
    scope: Option<ApiKeyScope>,
}

impl CreateApiKeyRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn scope(mut self, value: ApiKeyScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateApiKeyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateApiKeyRequestBuilder::name)
    pub fn build(self) -> Result<CreateApiKeyRequest, BuildError> {
        Ok(CreateApiKeyRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            scope: self.scope,
        })
    }
}

