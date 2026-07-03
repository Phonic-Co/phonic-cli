pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProjectsListResponse {
    #[serde(default)]
    pub projects: Vec<Project>,
}

impl ProjectsListResponse {
    pub fn builder() -> ProjectsListResponseBuilder {
        <ProjectsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectsListResponseBuilder {
    projects: Option<Vec<Project>>,
}

impl ProjectsListResponseBuilder {
    pub fn projects(mut self, value: Vec<Project>) -> Self {
        self.projects = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`projects`](ProjectsListResponseBuilder::projects)
    pub fn build(self) -> Result<ProjectsListResponse, BuildError> {
        Ok(ProjectsListResponse {
            projects: self.projects.ok_or_else(|| BuildError::missing_field("projects"))?,
        })
    }
}
