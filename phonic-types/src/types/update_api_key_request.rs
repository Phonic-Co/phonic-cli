pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateApiKeyRequest {
    /// The new name for the API key. Unique within the workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Replaces the key's scope. Must be within the caller's own scope.
    /// The caller can change a key only when that key's current scope is
    /// also within the caller's scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<ApiKeyScope>,
}

impl UpdateApiKeyRequest {
    pub fn builder() -> UpdateApiKeyRequestBuilder {
        <UpdateApiKeyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateApiKeyRequestBuilder {
    name: Option<String>,
    scope: Option<ApiKeyScope>,
}

impl UpdateApiKeyRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn scope(mut self, value: ApiKeyScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateApiKeyRequest`].
    pub fn build(self) -> Result<UpdateApiKeyRequest, BuildError> {
        Ok(UpdateApiKeyRequest {
            name: self.name,
            scope: self.scope,
        })
    }
}

