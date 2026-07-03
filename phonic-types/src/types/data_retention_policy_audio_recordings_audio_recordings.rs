pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DataRetentionPolicyAudioRecordingsAudioRecordings {
    /// Number of hours after which audio recordings are deleted. Null means audio recordings are retained indefinitely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_hours: Option<i64>,
}

impl DataRetentionPolicyAudioRecordingsAudioRecordings {
    pub fn builder() -> DataRetentionPolicyAudioRecordingsAudioRecordingsBuilder {
        <DataRetentionPolicyAudioRecordingsAudioRecordingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataRetentionPolicyAudioRecordingsAudioRecordingsBuilder {
    delete_after_hours: Option<i64>,
}

impl DataRetentionPolicyAudioRecordingsAudioRecordingsBuilder {
    pub fn delete_after_hours(mut self, value: i64) -> Self {
        self.delete_after_hours = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DataRetentionPolicyAudioRecordingsAudioRecordings`].
    pub fn build(self) -> Result<DataRetentionPolicyAudioRecordingsAudioRecordings, BuildError> {
        Ok(DataRetentionPolicyAudioRecordingsAudioRecordings {
            delete_after_hours: self.delete_after_hours,
        })
    }
}
