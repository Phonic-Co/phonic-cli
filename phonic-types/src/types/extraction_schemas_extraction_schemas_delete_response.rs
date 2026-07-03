pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExtractionSchemasDeleteResponse {
    /// Whether the extraction schema was deleted successfully.
    #[serde(default)]
    pub success: bool,
}

impl ExtractionSchemasDeleteResponse {
    pub fn builder() -> ExtractionSchemasDeleteResponseBuilder {
        <ExtractionSchemasDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExtractionSchemasDeleteResponseBuilder {
    success: Option<bool>,
}

impl ExtractionSchemasDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExtractionSchemasDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ExtractionSchemasDeleteResponseBuilder::success)
    pub fn build(self) -> Result<ExtractionSchemasDeleteResponse, BuildError> {
        Ok(ExtractionSchemasDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
