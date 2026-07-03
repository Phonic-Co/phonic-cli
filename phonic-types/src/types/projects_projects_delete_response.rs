pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProjectsDeleteResponse {
    /// Whether the project was deleted successfully.
    #[serde(default)]
    pub success: bool,
}

impl ProjectsDeleteResponse {
    pub fn builder() -> ProjectsDeleteResponseBuilder {
        <ProjectsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectsDeleteResponseBuilder {
    success: Option<bool>,
}

impl ProjectsDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ProjectsDeleteResponseBuilder::success)
    pub fn build(self) -> Result<ProjectsDeleteResponse, BuildError> {
        Ok(ProjectsDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
