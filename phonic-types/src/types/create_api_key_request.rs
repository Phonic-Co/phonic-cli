pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateApiKeyRequest {
    /// A name to identify the API key.
    #[serde(default)]
    pub name: String,
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
}

impl CreateApiKeyRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateApiKeyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateApiKeyRequestBuilder::name)
    pub fn build(self) -> Result<CreateApiKeyRequest, BuildError> {
        Ok(CreateApiKeyRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

