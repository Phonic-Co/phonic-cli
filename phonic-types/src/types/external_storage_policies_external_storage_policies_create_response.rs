pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExternalStoragePoliciesCreateResponse {
    /// The ID of the created external storage policy.
    #[serde(default)]
    pub id: String,
    /// The name of the created external storage policy.
    #[serde(default)]
    pub name: String,
}

impl ExternalStoragePoliciesCreateResponse {
    pub fn builder() -> ExternalStoragePoliciesCreateResponseBuilder {
        <ExternalStoragePoliciesCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalStoragePoliciesCreateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ExternalStoragePoliciesCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExternalStoragePoliciesCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExternalStoragePoliciesCreateResponseBuilder::id)
    /// - [`name`](ExternalStoragePoliciesCreateResponseBuilder::name)
    pub fn build(self) -> Result<ExternalStoragePoliciesCreateResponse, BuildError> {
        Ok(ExternalStoragePoliciesCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
