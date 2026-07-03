pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ConfigOptionsDataRetentionPolicy {
        ConfigOptionsDataRetentionPolicyZero(ConfigOptionsDataRetentionPolicyZero),

        ConfigOptionsDataRetentionPolicyAudioRecordings(ConfigOptionsDataRetentionPolicyAudioRecordings),
}

impl ConfigOptionsDataRetentionPolicy {
    pub fn is_config_options_data_retention_policy_zero(&self) -> bool {
        matches!(self, Self::ConfigOptionsDataRetentionPolicyZero(_))
    }

    pub fn is_config_options_data_retention_policy_audio_recordings(&self) -> bool {
        matches!(self, Self::ConfigOptionsDataRetentionPolicyAudioRecordings(_))
    }


    pub fn as_config_options_data_retention_policy_zero(&self) -> Option<&ConfigOptionsDataRetentionPolicyZero> {
        match self {
                    Self::ConfigOptionsDataRetentionPolicyZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_config_options_data_retention_policy_zero(self) -> Option<ConfigOptionsDataRetentionPolicyZero> {
        match self {
                    Self::ConfigOptionsDataRetentionPolicyZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_config_options_data_retention_policy_audio_recordings(&self) -> Option<&ConfigOptionsDataRetentionPolicyAudioRecordings> {
        match self {
                    Self::ConfigOptionsDataRetentionPolicyAudioRecordings(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_config_options_data_retention_policy_audio_recordings(self) -> Option<ConfigOptionsDataRetentionPolicyAudioRecordings> {
        match self {
                    Self::ConfigOptionsDataRetentionPolicyAudioRecordings(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ConfigOptionsDataRetentionPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigOptionsDataRetentionPolicyZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConfigOptionsDataRetentionPolicyAudioRecordings(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
