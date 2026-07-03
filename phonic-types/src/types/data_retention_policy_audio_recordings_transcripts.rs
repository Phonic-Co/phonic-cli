pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DataRetentionPolicyAudioRecordingsTranscripts {
    /// Number of hours after which transcripts are deleted. Null means transcripts are retained indefinitely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_hours: Option<i64>,
}

impl DataRetentionPolicyAudioRecordingsTranscripts {
    pub fn builder() -> DataRetentionPolicyAudioRecordingsTranscriptsBuilder {
        <DataRetentionPolicyAudioRecordingsTranscriptsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataRetentionPolicyAudioRecordingsTranscriptsBuilder {
    delete_after_hours: Option<i64>,
}

impl DataRetentionPolicyAudioRecordingsTranscriptsBuilder {
    pub fn delete_after_hours(mut self, value: i64) -> Self {
        self.delete_after_hours = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DataRetentionPolicyAudioRecordingsTranscripts`].
    pub fn build(self) -> Result<DataRetentionPolicyAudioRecordingsTranscripts, BuildError> {
        Ok(DataRetentionPolicyAudioRecordingsTranscripts {
            delete_after_hours: self.delete_after_hours,
        })
    }
}
