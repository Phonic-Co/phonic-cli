pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProjectsListEvalsResponse {
    #[serde(default)]
    pub evals: Vec<ConversationEval>,
}

impl ProjectsListEvalsResponse {
    pub fn builder() -> ProjectsListEvalsResponseBuilder {
        <ProjectsListEvalsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectsListEvalsResponseBuilder {
    evals: Option<Vec<ConversationEval>>,
}

impl ProjectsListEvalsResponseBuilder {
    pub fn evals(mut self, value: Vec<ConversationEval>) -> Self {
        self.evals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProjectsListEvalsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evals`](ProjectsListEvalsResponseBuilder::evals)
    pub fn build(self) -> Result<ProjectsListEvalsResponse, BuildError> {
        Ok(ProjectsListEvalsResponse {
            evals: self.evals.ok_or_else(|| BuildError::missing_field("evals"))?,
        })
    }
}
