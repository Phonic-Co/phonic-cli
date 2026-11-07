pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExternalStoragePoliciesDeleteResponse {
    /// Whether the external storage policy was deleted successfully.
    #[serde(default)]
    pub success: bool,
}

impl ExternalStoragePoliciesDeleteResponse {
    pub fn builder() -> ExternalStoragePoliciesDeleteResponseBuilder {
        <ExternalStoragePoliciesDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalStoragePoliciesDeleteResponseBuilder {
    success: Option<bool>,
}

impl ExternalStoragePoliciesDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExternalStoragePoliciesDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ExternalStoragePoliciesDeleteResponseBuilder::success)
    pub fn build(self) -> Result<ExternalStoragePoliciesDeleteResponse, BuildError> {
        Ok(ExternalStoragePoliciesDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
