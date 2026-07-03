pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentRequestPronunciationDictionaryItem {
    #[serde(default)]
    pub word: String,
    #[serde(default)]
    pub pronunciation: String,
}

impl CreateAgentRequestPronunciationDictionaryItem {
    pub fn builder() -> CreateAgentRequestPronunciationDictionaryItemBuilder {
        <CreateAgentRequestPronunciationDictionaryItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentRequestPronunciationDictionaryItemBuilder {
    word: Option<String>,
    pronunciation: Option<String>,
}

impl CreateAgentRequestPronunciationDictionaryItemBuilder {
    pub fn word(mut self, value: impl Into<String>) -> Self {
        self.word = Some(value.into());
        self
    }

    pub fn pronunciation(mut self, value: impl Into<String>) -> Self {
        self.pronunciation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentRequestPronunciationDictionaryItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`word`](CreateAgentRequestPronunciationDictionaryItemBuilder::word)
    /// - [`pronunciation`](CreateAgentRequestPronunciationDictionaryItemBuilder::pronunciation)
    pub fn build(self) -> Result<CreateAgentRequestPronunciationDictionaryItem, BuildError> {
        Ok(CreateAgentRequestPronunciationDictionaryItem {
            word: self.word.ok_or_else(|| BuildError::missing_field("word"))?,
            pronunciation: self.pronunciation.ok_or_else(|| BuildError::missing_field("pronunciation"))?,
        })
    }
}
