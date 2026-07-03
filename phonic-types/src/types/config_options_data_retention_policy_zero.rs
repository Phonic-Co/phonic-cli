pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Zero data retention mode. No transcripts or audio recordings are retained.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct ConfigOptionsDataRetentionPolicyZero {
    /// When `true`, no transcripts or audio recordings are retained.
    pub zero_data_retention: bool,
}

impl ConfigOptionsDataRetentionPolicyZero {
    pub fn builder() -> ConfigOptionsDataRetentionPolicyZeroBuilder {
        <ConfigOptionsDataRetentionPolicyZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigOptionsDataRetentionPolicyZeroBuilder {
    zero_data_retention: Option<bool>,
}

impl ConfigOptionsDataRetentionPolicyZeroBuilder {
    pub fn zero_data_retention(mut self, value: bool) -> Self {
        self.zero_data_retention = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigOptionsDataRetentionPolicyZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`zero_data_retention`](ConfigOptionsDataRetentionPolicyZeroBuilder::zero_data_retention)
    pub fn build(self) -> Result<ConfigOptionsDataRetentionPolicyZero, BuildError> {
        Ok(ConfigOptionsDataRetentionPolicyZero {
            zero_data_retention: self.zero_data_retention.ok_or_else(|| BuildError::missing_field("zero_data_retention"))?,
        })
    }
}
