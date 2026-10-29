pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExternalStoragePoliciesDeleteQueryRequest {
    /// The name of the project containing the external storage policy. Only used when `nameOrId` is a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ExternalStoragePoliciesDeleteQueryRequest {
    pub fn builder() -> ExternalStoragePoliciesDeleteQueryRequestBuilder {
        <ExternalStoragePoliciesDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalStoragePoliciesDeleteQueryRequestBuilder {
    project: Option<String>,
}

impl ExternalStoragePoliciesDeleteQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExternalStoragePoliciesDeleteQueryRequest`].
    pub fn build(self) -> Result<ExternalStoragePoliciesDeleteQueryRequest, BuildError> {
        Ok(ExternalStoragePoliciesDeleteQueryRequest {
            project: self.project,
        })
    }
}

