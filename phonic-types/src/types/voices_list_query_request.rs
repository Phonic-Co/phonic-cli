pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VoicesListQueryRequest {
    /// The model to get voices for.
    pub model: String,
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
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](VoicesListQueryRequestBuilder::model)
    pub fn build(self) -> Result<VoicesListQueryRequest, BuildError> {
        Ok(VoicesListQueryRequest {
            model: self.model.ok_or_else(|| BuildError::missing_field("model"))?,
        })
    }
}

