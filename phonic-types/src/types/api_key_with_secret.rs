pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeyWithSecret {
    /// The API key ID.
    #[serde(default)]
    pub id: String,
    /// The name of the API key.
    #[serde(default)]
    pub name: String,
    /// The API key secret. Only returned once, when the key is created or rotated.
    #[serde(default)]
    pub api_key: String,
}

impl ApiKeyWithSecret {
    pub fn builder() -> ApiKeyWithSecretBuilder {
        <ApiKeyWithSecretBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyWithSecretBuilder {
    id: Option<String>,
    name: Option<String>,
    api_key: Option<String>,
}

impl ApiKeyWithSecretBuilder {
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

    /// Consumes the builder and constructs a [`ApiKeyWithSecret`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ApiKeyWithSecretBuilder::id)
    /// - [`name`](ApiKeyWithSecretBuilder::name)
    /// - [`api_key`](ApiKeyWithSecretBuilder::api_key)
    pub fn build(self) -> Result<ApiKeyWithSecret, BuildError> {
        Ok(ApiKeyWithSecret {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            api_key: self.api_key.ok_or_else(|| BuildError::missing_field("api_key"))?,
        })
    }
}
