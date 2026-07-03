pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceUpdateResponse {
    #[serde(default)]
    pub success: bool,
}

impl WorkspaceUpdateResponse {
    pub fn builder() -> WorkspaceUpdateResponseBuilder {
        <WorkspaceUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceUpdateResponseBuilder {
    success: Option<bool>,
}

impl WorkspaceUpdateResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](WorkspaceUpdateResponseBuilder::success)
    pub fn build(self) -> Result<WorkspaceUpdateResponse, BuildError> {
        Ok(WorkspaceUpdateResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
