pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExternalStoragePoliciesListQueryRequest {
    /// The name of the project to list external storage policies for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ExternalStoragePoliciesListQueryRequest {
    pub fn builder() -> ExternalStoragePoliciesListQueryRequestBuilder {
        <ExternalStoragePoliciesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalStoragePoliciesListQueryRequestBuilder {
    project: Option<String>,
}

impl ExternalStoragePoliciesListQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExternalStoragePoliciesListQueryRequest`].
    pub fn build(self) -> Result<ExternalStoragePoliciesListQueryRequest, BuildError> {
        Ok(ExternalStoragePoliciesListQueryRequest {
            project: self.project,
        })
    }
}

