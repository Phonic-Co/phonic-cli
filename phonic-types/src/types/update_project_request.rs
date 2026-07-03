pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateProjectRequest {
    /// The name of the project. Can only contain lowercase letters, numbers and hyphens. Must be unique within the workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The name of the new project's default agent. Set to `null` to remove the default agent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_agent: Option<String>,
    /// Maximum number of concurrent conversations allowed for this project. When `null`, the workspace `max_active_conversations` limit is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_active_conversations: Option<i64>,
}

impl UpdateProjectRequest {
    pub fn builder() -> UpdateProjectRequestBuilder {
        <UpdateProjectRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateProjectRequestBuilder {
    name: Option<String>,
    default_agent: Option<String>,
    max_active_conversations: Option<i64>,
}

impl UpdateProjectRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn default_agent(mut self, value: impl Into<String>) -> Self {
        self.default_agent = Some(value.into());
        self
    }

    pub fn max_active_conversations(mut self, value: i64) -> Self {
        self.max_active_conversations = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateProjectRequest`].
    pub fn build(self) -> Result<UpdateProjectRequest, BuildError> {
        Ok(UpdateProjectRequest {
            name: self.name,
            default_agent: self.default_agent,
            max_active_conversations: self.max_active_conversations,
        })
    }
}

