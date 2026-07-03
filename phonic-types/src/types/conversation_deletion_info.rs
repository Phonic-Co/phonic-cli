pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Information about when transcripts and audio recordings are or were scheduled to be deleted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationDeletionInfo {
    /// When the transcripts were deleted. `null` if not deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub transcripts_deleted_at: Option<DateTime<FixedOffset>>,
    /// When the audio recordings were deleted. `null` if not deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub audio_recordings_deleted_at: Option<DateTime<FixedOffset>>,
}

impl ConversationDeletionInfo {
    pub fn builder() -> ConversationDeletionInfoBuilder {
        <ConversationDeletionInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationDeletionInfoBuilder {
    transcripts_deleted_at: Option<DateTime<FixedOffset>>,
    audio_recordings_deleted_at: Option<DateTime<FixedOffset>>,
}

impl ConversationDeletionInfoBuilder {
    pub fn transcripts_deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.transcripts_deleted_at = Some(value);
        self
    }

    pub fn audio_recordings_deleted_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.audio_recordings_deleted_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationDeletionInfo`].
    pub fn build(self) -> Result<ConversationDeletionInfo, BuildError> {
        Ok(ConversationDeletionInfo {
            transcripts_deleted_at: self.transcripts_deleted_at,
            audio_recordings_deleted_at: self.audio_recordings_deleted_at,
        })
    }
}
