pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The evaluation prompt information.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationEvalPromptInfo {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl ConversationEvalPromptInfo {
    pub fn builder() -> ConversationEvalPromptInfoBuilder {
        <ConversationEvalPromptInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationEvalPromptInfoBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ConversationEvalPromptInfoBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationEvalPromptInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationEvalPromptInfoBuilder::id)
    /// - [`name`](ConversationEvalPromptInfoBuilder::name)
    pub fn build(self) -> Result<ConversationEvalPromptInfo, BuildError> {
        Ok(ConversationEvalPromptInfo {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
