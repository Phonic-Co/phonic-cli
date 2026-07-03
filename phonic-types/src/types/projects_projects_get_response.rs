pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProjectsGetResponse {
    #[serde(default)]
    pub project: Project,
}

impl ProjectsGetResponse {
    pub fn builder() -> ProjectsGetResponseBuilder {
        <ProjectsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectsGetResponseBuilder {
    project: Option<Project>,
}

impl ProjectsGetResponseBuilder {
    pub fn project(mut self, value: Project) -> Self {
        self.project = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`project`](ProjectsGetResponseBuilder::project)
    pub fn build(self) -> Result<ProjectsGetResponse, BuildError> {
        Ok(ProjectsGetResponse {
            project: self.project.ok_or_else(|| BuildError::missing_field("project"))?,
        })
    }
}
