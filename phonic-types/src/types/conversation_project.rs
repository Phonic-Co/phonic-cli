pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The project associated with the conversation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationProject {
    /// The ID of the project.
    #[serde(default)]
    pub id: String,
    /// The name of the project.
    #[serde(default)]
    pub name: String,
}

impl ConversationProject {
    pub fn builder() -> ConversationProjectBuilder {
        <ConversationProjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationProjectBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ConversationProjectBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationProject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationProjectBuilder::id)
    /// - [`name`](ConversationProjectBuilder::name)
    pub fn build(self) -> Result<ConversationProject, BuildError> {
        Ok(ConversationProject {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
