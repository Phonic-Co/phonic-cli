pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigOptionsDataRetentionPolicyAudioRecordingsTranscripts {
    /// Number of hours after which transcripts are deleted. When `null`, transcripts are retained indefinitely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_hours: Option<i64>,
}

impl ConfigOptionsDataRetentionPolicyAudioRecordingsTranscripts {
    pub fn builder() -> ConfigOptionsDataRetentionPolicyAudioRecordingsTranscriptsBuilder {
        <ConfigOptionsDataRetentionPolicyAudioRecordingsTranscriptsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigOptionsDataRetentionPolicyAudioRecordingsTranscriptsBuilder {
    delete_after_hours: Option<i64>,
}

impl ConfigOptionsDataRetentionPolicyAudioRecordingsTranscriptsBuilder {
    pub fn delete_after_hours(mut self, value: i64) -> Self {
        self.delete_after_hours = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigOptionsDataRetentionPolicyAudioRecordingsTranscripts`].
    pub fn build(self) -> Result<ConfigOptionsDataRetentionPolicyAudioRecordingsTranscripts, BuildError> {
        Ok(ConfigOptionsDataRetentionPolicyAudioRecordingsTranscripts {
            delete_after_hours: self.delete_after_hours,
        })
    }
}
