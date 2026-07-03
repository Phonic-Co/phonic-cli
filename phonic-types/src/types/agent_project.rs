pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The project the agent belongs to.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentProject {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl AgentProject {
    pub fn builder() -> AgentProjectBuilder {
        <AgentProjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentProjectBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl AgentProjectBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentProject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentProjectBuilder::id)
    /// - [`name`](AgentProjectBuilder::name)
    pub fn build(self) -> Result<AgentProject, BuildError> {
        Ok(AgentProject {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
