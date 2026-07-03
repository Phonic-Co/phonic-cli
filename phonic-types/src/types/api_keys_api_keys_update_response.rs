pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiKeysUpdateResponse {
    #[serde(default)]
    pub success: bool,
}

impl ApiKeysUpdateResponse {
    pub fn builder() -> ApiKeysUpdateResponseBuilder {
        <ApiKeysUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysUpdateResponseBuilder {
    success: Option<bool>,
}

impl ApiKeysUpdateResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeysUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ApiKeysUpdateResponseBuilder::success)
    pub fn build(self) -> Result<ApiKeysUpdateResponse, BuildError> {
        Ok(ApiKeysUpdateResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
