pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExternalStoragePoliciesGetResponse {
    pub external_storage_policy: ExternalStoragePolicy,
}

impl ExternalStoragePoliciesGetResponse {
    pub fn builder() -> ExternalStoragePoliciesGetResponseBuilder {
        <ExternalStoragePoliciesGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalStoragePoliciesGetResponseBuilder {
    external_storage_policy: Option<ExternalStoragePolicy>,
}

impl ExternalStoragePoliciesGetResponseBuilder {
    pub fn external_storage_policy(mut self, value: ExternalStoragePolicy) -> Self {
        self.external_storage_policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExternalStoragePoliciesGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_storage_policy`](ExternalStoragePoliciesGetResponseBuilder::external_storage_policy)
    pub fn build(self) -> Result<ExternalStoragePoliciesGetResponse, BuildError> {
        Ok(ExternalStoragePoliciesGetResponse {
            external_storage_policy: self.external_storage_policy.ok_or_else(|| BuildError::missing_field("external_storage_policy"))?,
        })
    }
}
