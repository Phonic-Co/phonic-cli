pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum DataRetentionPolicy {
        DataRetentionPolicyZero(DataRetentionPolicyZero),

        DataRetentionPolicyAudioRecordings(DataRetentionPolicyAudioRecordings),
}

impl DataRetentionPolicy {
    pub fn is_data_retention_policy_zero(&self) -> bool {
        matches!(self, Self::DataRetentionPolicyZero(_))
    }

    pub fn is_data_retention_policy_audio_recordings(&self) -> bool {
        matches!(self, Self::DataRetentionPolicyAudioRecordings(_))
    }


    pub fn as_data_retention_policy_zero(&self) -> Option<&DataRetentionPolicyZero> {
        match self {
                    Self::DataRetentionPolicyZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_data_retention_policy_zero(self) -> Option<DataRetentionPolicyZero> {
        match self {
                    Self::DataRetentionPolicyZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_data_retention_policy_audio_recordings(&self) -> Option<&DataRetentionPolicyAudioRecordings> {
        match self {
                    Self::DataRetentionPolicyAudioRecordings(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_data_retention_policy_audio_recordings(self) -> Option<DataRetentionPolicyAudioRecordings> {
        match self {
                    Self::DataRetentionPolicyAudioRecordings(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for DataRetentionPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DataRetentionPolicyZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::DataRetentionPolicyAudioRecordings(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
