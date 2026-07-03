pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The project's default agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProjectDefaultAgent {
    /// The ID of the project's default agent.
    #[serde(default)]
    pub id: String,
    /// The name of the project's default agent.
    #[serde(default)]
    pub name: String,
}

impl ProjectDefaultAgent {
    pub fn builder() -> ProjectDefaultAgentBuilder {
        <ProjectDefaultAgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectDefaultAgentBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ProjectDefaultAgentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProjectDefaultAgent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProjectDefaultAgentBuilder::id)
    /// - [`name`](ProjectDefaultAgentBuilder::name)
    pub fn build(self) -> Result<ProjectDefaultAgent, BuildError> {
        Ok(ProjectDefaultAgent {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
