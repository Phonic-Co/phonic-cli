pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for get
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExternalStoragePoliciesGetQueryRequest {
    /// The name of the project containing the external storage policy. Only used when `nameOrId` is a name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl ExternalStoragePoliciesGetQueryRequest {
    pub fn builder() -> ExternalStoragePoliciesGetQueryRequestBuilder {
        <ExternalStoragePoliciesGetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalStoragePoliciesGetQueryRequestBuilder {
    project: Option<String>,
}

impl ExternalStoragePoliciesGetQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExternalStoragePoliciesGetQueryRequest`].
    pub fn build(self) -> Result<ExternalStoragePoliciesGetQueryRequest, BuildError> {
        Ok(ExternalStoragePoliciesGetQueryRequest {
            project: self.project,
        })
    }
}

