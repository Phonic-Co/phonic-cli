pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksDeleteResponse {
    #[serde(default)]
    pub success: bool,
}

impl WebhooksDeleteResponse {
    pub fn builder() -> WebhooksDeleteResponseBuilder {
        <WebhooksDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksDeleteResponseBuilder {
    success: Option<bool>,
}

impl WebhooksDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](WebhooksDeleteResponseBuilder::success)
    pub fn build(self) -> Result<WebhooksDeleteResponse, BuildError> {
        Ok(WebhooksDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
