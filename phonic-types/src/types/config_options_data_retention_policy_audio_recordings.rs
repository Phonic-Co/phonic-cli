pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Standard data retention with configurable deletion windows.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ConfigOptionsDataRetentionPolicyAudioRecordings {
    /// Must be `false` for standard data retention.
    pub zero_data_retention: bool,
    #[serde(default)]
    pub transcripts: ConfigOptionsDataRetentionPolicyAudioRecordingsTranscripts,
    #[serde(default)]
    pub audio_recordings: ConfigOptionsDataRetentionPolicyAudioRecordingsAudioRecordings,
}

impl ConfigOptionsDataRetentionPolicyAudioRecordings {
    pub fn builder() -> ConfigOptionsDataRetentionPolicyAudioRecordingsBuilder {
        <ConfigOptionsDataRetentionPolicyAudioRecordingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigOptionsDataRetentionPolicyAudioRecordingsBuilder {
    zero_data_retention: Option<bool>,
    transcripts: Option<ConfigOptionsDataRetentionPolicyAudioRecordingsTranscripts>,
    audio_recordings: Option<ConfigOptionsDataRetentionPolicyAudioRecordingsAudioRecordings>,
}

impl ConfigOptionsDataRetentionPolicyAudioRecordingsBuilder {
    pub fn zero_data_retention(mut self, value: bool) -> Self {
        self.zero_data_retention = Some(value);
        self
    }

    pub fn transcripts(mut self, value: ConfigOptionsDataRetentionPolicyAudioRecordingsTranscripts) -> Self {
        self.transcripts = Some(value);
        self
    }

    pub fn audio_recordings(mut self, value: ConfigOptionsDataRetentionPolicyAudioRecordingsAudioRecordings) -> Self {
        self.audio_recordings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigOptionsDataRetentionPolicyAudioRecordings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`zero_data_retention`](ConfigOptionsDataRetentionPolicyAudioRecordingsBuilder::zero_data_retention)
    /// - [`transcripts`](ConfigOptionsDataRetentionPolicyAudioRecordingsBuilder::transcripts)
    /// - [`audio_recordings`](ConfigOptionsDataRetentionPolicyAudioRecordingsBuilder::audio_recordings)
    pub fn build(self) -> Result<ConfigOptionsDataRetentionPolicyAudioRecordings, BuildError> {
        Ok(ConfigOptionsDataRetentionPolicyAudioRecordings {
            zero_data_retention: self.zero_data_retention.ok_or_else(|| BuildError::missing_field("zero_data_retention"))?,
            transcripts: self.transcripts.ok_or_else(|| BuildError::missing_field("transcripts"))?,
            audio_recordings: self.audio_recordings.ok_or_else(|| BuildError::missing_field("audio_recordings"))?,
        })
    }
}
