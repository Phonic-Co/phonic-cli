pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeysDeleteResponse {
    #[serde(default)]
    pub success: bool,
}

impl ApiKeysDeleteResponse {
    pub fn builder() -> ApiKeysDeleteResponseBuilder {
        <ApiKeysDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysDeleteResponseBuilder {
    success: Option<bool>,
}

impl ApiKeysDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeysDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ApiKeysDeleteResponseBuilder::success)
    pub fn build(self) -> Result<ApiKeysDeleteResponse, BuildError> {
        Ok(ApiKeysDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
