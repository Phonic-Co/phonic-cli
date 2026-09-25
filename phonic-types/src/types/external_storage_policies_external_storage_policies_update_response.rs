pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExternalStoragePoliciesUpdateResponse {
    /// Whether the external storage policy was updated successfully.
    #[serde(default)]
    pub success: bool,
}

impl ExternalStoragePoliciesUpdateResponse {
    pub fn builder() -> ExternalStoragePoliciesUpdateResponseBuilder {
        <ExternalStoragePoliciesUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalStoragePoliciesUpdateResponseBuilder {
    success: Option<bool>,
}

impl ExternalStoragePoliciesUpdateResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExternalStoragePoliciesUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ExternalStoragePoliciesUpdateResponseBuilder::success)
    pub fn build(self) -> Result<ExternalStoragePoliciesUpdateResponse, BuildError> {
        Ok(ExternalStoragePoliciesUpdateResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
