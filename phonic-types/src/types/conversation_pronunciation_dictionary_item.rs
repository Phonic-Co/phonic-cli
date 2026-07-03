pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationPronunciationDictionaryItem {
    #[serde(default)]
    pub word: String,
    #[serde(default)]
    pub pronunciation: String,
}

impl ConversationPronunciationDictionaryItem {
    pub fn builder() -> ConversationPronunciationDictionaryItemBuilder {
        <ConversationPronunciationDictionaryItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationPronunciationDictionaryItemBuilder {
    word: Option<String>,
    pronunciation: Option<String>,
}

impl ConversationPronunciationDictionaryItemBuilder {
    pub fn word(mut self, value: impl Into<String>) -> Self {
        self.word = Some(value.into());
        self
    }

    pub fn pronunciation(mut self, value: impl Into<String>) -> Self {
        self.pronunciation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationPronunciationDictionaryItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`word`](ConversationPronunciationDictionaryItemBuilder::word)
    /// - [`pronunciation`](ConversationPronunciationDictionaryItemBuilder::pronunciation)
    pub fn build(self) -> Result<ConversationPronunciationDictionaryItem, BuildError> {
        Ok(ConversationPronunciationDictionaryItem {
            word: self.word.ok_or_else(|| BuildError::missing_field("word"))?,
            pronunciation: self.pronunciation.ok_or_else(|| BuildError::missing_field("pronunciation"))?,
        })
    }
}
