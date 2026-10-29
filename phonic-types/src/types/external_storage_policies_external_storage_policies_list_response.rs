pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExternalStoragePoliciesListResponse {
    #[serde(default)]
    pub external_storage_policies: Vec<ExternalStoragePolicy>,
}

impl ExternalStoragePoliciesListResponse {
    pub fn builder() -> ExternalStoragePoliciesListResponseBuilder {
        <ExternalStoragePoliciesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalStoragePoliciesListResponseBuilder {
    external_storage_policies: Option<Vec<ExternalStoragePolicy>>,
}

impl ExternalStoragePoliciesListResponseBuilder {
    pub fn external_storage_policies(mut self, value: Vec<ExternalStoragePolicy>) -> Self {
        self.external_storage_policies = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExternalStoragePoliciesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_storage_policies`](ExternalStoragePoliciesListResponseBuilder::external_storage_policies)
    pub fn build(self) -> Result<ExternalStoragePoliciesListResponse, BuildError> {
        Ok(ExternalStoragePoliciesListResponse {
            external_storage_policies: self.external_storage_policies.ok_or_else(|| BuildError::missing_field("external_storage_policies"))?,
        })
    }
}
