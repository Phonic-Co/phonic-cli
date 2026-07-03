pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigOptionsDataRetentionPolicyAudioRecordingsAudioRecordings {
    /// Number of hours after which audio recordings are deleted. When `null`, audio recordings are retained indefinitely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_after_hours: Option<i64>,
}

impl ConfigOptionsDataRetentionPolicyAudioRecordingsAudioRecordings {
    pub fn builder() -> ConfigOptionsDataRetentionPolicyAudioRecordingsAudioRecordingsBuilder {
        <ConfigOptionsDataRetentionPolicyAudioRecordingsAudioRecordingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigOptionsDataRetentionPolicyAudioRecordingsAudioRecordingsBuilder {
    delete_after_hours: Option<i64>,
}

impl ConfigOptionsDataRetentionPolicyAudioRecordingsAudioRecordingsBuilder {
    pub fn delete_after_hours(mut self, value: i64) -> Self {
        self.delete_after_hours = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigOptionsDataRetentionPolicyAudioRecordingsAudioRecordings`].
    pub fn build(self) -> Result<ConfigOptionsDataRetentionPolicyAudioRecordingsAudioRecordings, BuildError> {
        Ok(ConfigOptionsDataRetentionPolicyAudioRecordingsAudioRecordings {
            delete_after_hours: self.delete_after_hours,
        })
    }
}
