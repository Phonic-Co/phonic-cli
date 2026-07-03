pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProjectsCreateResponse {
    /// The ID of the created project.
    #[serde(default)]
    pub id: String,
    /// The name of the created project.
    #[serde(default)]
    pub name: String,
}

impl ProjectsCreateResponse {
    pub fn builder() -> ProjectsCreateResponseBuilder {
        <ProjectsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectsCreateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ProjectsCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProjectsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProjectsCreateResponseBuilder::id)
    /// - [`name`](ProjectsCreateResponseBuilder::name)
    pub fn build(self) -> Result<ProjectsCreateResponse, BuildError> {
        Ok(ProjectsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
