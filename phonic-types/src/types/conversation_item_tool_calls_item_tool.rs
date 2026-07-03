pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationItemToolCallsItemTool {
    /// The tool ID.
    #[serde(default)]
    pub id: String,
    /// The tool name.
    #[serde(default)]
    pub name: String,
}

impl ConversationItemToolCallsItemTool {
    pub fn builder() -> ConversationItemToolCallsItemToolBuilder {
        <ConversationItemToolCallsItemToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationItemToolCallsItemToolBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ConversationItemToolCallsItemToolBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationItemToolCallsItemTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationItemToolCallsItemToolBuilder::id)
    /// - [`name`](ConversationItemToolCallsItemToolBuilder::name)
    pub fn build(self) -> Result<ConversationItemToolCallsItemTool, BuildError> {
        Ok(ConversationItemToolCallsItemTool {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
