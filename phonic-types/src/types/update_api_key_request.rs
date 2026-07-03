pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateApiKeyRequest {
    /// The new name for the API key.
    #[serde(default)]
    pub name: String,
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
}

impl UpdateApiKeyRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateApiKeyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateApiKeyRequestBuilder::name)
    pub fn build(self) -> Result<UpdateApiKeyRequest, BuildError> {
        Ok(UpdateApiKeyRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

