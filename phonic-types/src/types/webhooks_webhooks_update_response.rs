pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhooksUpdateResponse {
    #[serde(default)]
    pub success: bool,
}

impl WebhooksUpdateResponse {
    pub fn builder() -> WebhooksUpdateResponseBuilder {
        <WebhooksUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhooksUpdateResponseBuilder {
    success: Option<bool>,
}

impl WebhooksUpdateResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebhooksUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](WebhooksUpdateResponseBuilder::success)
    pub fn build(self) -> Result<WebhooksUpdateResponse, BuildError> {
        Ok(WebhooksUpdateResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
