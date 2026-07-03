pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProjectsCreateEvalPromptResponse {
    /// The ID of the created evaluation prompt.
    #[serde(default)]
    pub id: String,
}

impl ProjectsCreateEvalPromptResponse {
    pub fn builder() -> ProjectsCreateEvalPromptResponseBuilder {
        <ProjectsCreateEvalPromptResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectsCreateEvalPromptResponseBuilder {
    id: Option<String>,
}

impl ProjectsCreateEvalPromptResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProjectsCreateEvalPromptResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProjectsCreateEvalPromptResponseBuilder::id)
    pub fn build(self) -> Result<ProjectsCreateEvalPromptResponse, BuildError> {
        Ok(ProjectsCreateEvalPromptResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
