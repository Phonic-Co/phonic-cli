pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The agent associated with the conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationAgent {
    /// The ID of the agent.
    #[serde(default)]
    pub id: String,
    /// The name of the agent.
    #[serde(default)]
    pub name: String,
    /// Whether the agent has been deleted.
    #[serde(default)]
    pub is_deleted: bool,
}

impl ConversationAgent {
    pub fn builder() -> ConversationAgentBuilder {
        <ConversationAgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationAgentBuilder {
    id: Option<String>,
    name: Option<String>,
    is_deleted: Option<bool>,
}

impl ConversationAgentBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn is_deleted(mut self, value: bool) -> Self {
        self.is_deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationAgent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationAgentBuilder::id)
    /// - [`name`](ConversationAgentBuilder::name)
    /// - [`is_deleted`](ConversationAgentBuilder::is_deleted)
    pub fn build(self) -> Result<ConversationAgent, BuildError> {
        Ok(ConversationAgent {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            is_deleted: self.is_deleted.ok_or_else(|| BuildError::missing_field("is_deleted"))?,
        })
    }
}
