pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OutboundCallConfigPronunciationDictionaryItem {
    #[serde(default)]
    pub word: String,
    #[serde(default)]
    pub pronunciation: String,
}

impl OutboundCallConfigPronunciationDictionaryItem {
    pub fn builder() -> OutboundCallConfigPronunciationDictionaryItemBuilder {
        <OutboundCallConfigPronunciationDictionaryItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutboundCallConfigPronunciationDictionaryItemBuilder {
    word: Option<String>,
    pronunciation: Option<String>,
}

impl OutboundCallConfigPronunciationDictionaryItemBuilder {
    pub fn word(mut self, value: impl Into<String>) -> Self {
        self.word = Some(value.into());
        self
    }

    pub fn pronunciation(mut self, value: impl Into<String>) -> Self {
        self.pronunciation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OutboundCallConfigPronunciationDictionaryItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`word`](OutboundCallConfigPronunciationDictionaryItemBuilder::word)
    /// - [`pronunciation`](OutboundCallConfigPronunciationDictionaryItemBuilder::pronunciation)
    pub fn build(self) -> Result<OutboundCallConfigPronunciationDictionaryItem, BuildError> {
        Ok(OutboundCallConfigPronunciationDictionaryItem {
            word: self.word.ok_or_else(|| BuildError::missing_field("word"))?,
            pronunciation: self.pronunciation.ok_or_else(|| BuildError::missing_field("pronunciation"))?,
        })
    }
}
