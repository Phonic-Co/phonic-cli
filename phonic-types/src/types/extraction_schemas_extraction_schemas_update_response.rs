pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExtractionSchemasUpdateResponse {
    /// Whether the extraction schema was updated successfully.
    #[serde(default)]
    pub success: bool,
}

impl ExtractionSchemasUpdateResponse {
    pub fn builder() -> ExtractionSchemasUpdateResponseBuilder {
        <ExtractionSchemasUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractionSchemasUpdateResponseBuilder {
    success: Option<bool>,
}

impl ExtractionSchemasUpdateResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExtractionSchemasUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ExtractionSchemasUpdateResponseBuilder::success)
    pub fn build(self) -> Result<ExtractionSchemasUpdateResponse, BuildError> {
        Ok(ExtractionSchemasUpdateResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
