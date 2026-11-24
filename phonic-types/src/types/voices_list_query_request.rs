pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoicesListQueryRequest {
    /// Ignored. This parameter is deprecated and can be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl VoicesListQueryRequest {
    pub fn builder() -> VoicesListQueryRequestBuilder {
        <VoicesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoicesListQueryRequestBuilder {
    model: Option<String>,
}

impl VoicesListQueryRequestBuilder {
    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VoicesListQueryRequest`].
    pub fn build(self) -> Result<VoicesListQueryRequest, BuildError> {
        Ok(VoicesListQueryRequest {
            model: self.model,
        })
    }
}

