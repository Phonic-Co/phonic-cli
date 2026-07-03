pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Project {
    /// The ID of the project.
    #[serde(default)]
    pub id: String,
    /// The name of the project.
    #[serde(default)]
    pub name: String,
    /// The project's default agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_agent: Option<ProjectDefaultAgent>,
    /// Number of conversations currently in progress for this project.
    #[serde(default)]
    pub active_conversations: i64,
    /// Maximum number of concurrent conversations allowed for this project. When `null`, the workspace `max_active_conversations` limit is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_active_conversations: Option<i64>,
}

impl Project {
    pub fn builder() -> ProjectBuilder {
        <ProjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProjectBuilder {
    id: Option<String>,
    name: Option<String>,
    default_agent: Option<ProjectDefaultAgent>,
    active_conversations: Option<i64>,
    max_active_conversations: Option<i64>,
}

impl ProjectBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn default_agent(mut self, value: ProjectDefaultAgent) -> Self {
        self.default_agent = Some(value);
        self
    }

    pub fn active_conversations(mut self, value: i64) -> Self {
        self.active_conversations = Some(value);
        self
    }

    pub fn max_active_conversations(mut self, value: i64) -> Self {
        self.max_active_conversations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Project`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProjectBuilder::id)
    /// - [`name`](ProjectBuilder::name)
    /// - [`active_conversations`](ProjectBuilder::active_conversations)
    pub fn build(self) -> Result<Project, BuildError> {
        Ok(Project {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            default_agent: self.default_agent,
            active_conversations: self.active_conversations.ok_or_else(|| BuildError::missing_field("active_conversations"))?,
            max_active_conversations: self.max_active_conversations,
        })
    }
}
