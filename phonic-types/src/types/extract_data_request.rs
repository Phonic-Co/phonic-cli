pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExtractDataRequest {
    /// ID of the extraction schema to use.
    #[serde(default)]
    pub schema_id: String,
}

impl ExtractDataRequest {
    pub fn builder() -> ExtractDataRequestBuilder {
        <ExtractDataRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractDataRequestBuilder {
    schema_id: Option<String>,
}

impl ExtractDataRequestBuilder {
    pub fn schema_id(mut self, value: impl Into<String>) -> Self {
        self.schema_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExtractDataRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schema_id`](ExtractDataRequestBuilder::schema_id)
    pub fn build(self) -> Result<ExtractDataRequest, BuildError> {
        Ok(ExtractDataRequest {
            schema_id: self.schema_id.ok_or_else(|| BuildError::missing_field("schema_id"))?,
        })
    }
}

