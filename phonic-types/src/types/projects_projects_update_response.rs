pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProjectsUpdateResponse {
    /// Whether the project was updated successfully.
    #[serde(default)]
    pub success: bool,
}

impl ProjectsUpdateResponse {
    pub fn builder() -> ProjectsUpdateResponseBuilder {
        <ProjectsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectsUpdateResponseBuilder {
    success: Option<bool>,
}

impl ProjectsUpdateResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ProjectsUpdateResponseBuilder::success)
    pub fn build(self) -> Result<ProjectsUpdateResponse, BuildError> {
        Ok(ProjectsUpdateResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
