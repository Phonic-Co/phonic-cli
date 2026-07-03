pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Zero data retention mode. No transcripts or audio recordings are retained.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct DataRetentionPolicyZero {
    /// When `true`, no transcripts or audio recordings are retained.
    pub zero_data_retention: bool,
}

impl DataRetentionPolicyZero {
    pub fn builder() -> DataRetentionPolicyZeroBuilder {
        <DataRetentionPolicyZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DataRetentionPolicyZeroBuilder {
    zero_data_retention: Option<bool>,
}

impl DataRetentionPolicyZeroBuilder {
    pub fn zero_data_retention(mut self, value: bool) -> Self {
        self.zero_data_retention = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DataRetentionPolicyZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`zero_data_retention`](DataRetentionPolicyZeroBuilder::zero_data_retention)
    pub fn build(self) -> Result<DataRetentionPolicyZero, BuildError> {
        Ok(DataRetentionPolicyZero {
            zero_data_retention: self.zero_data_retention.ok_or_else(|| BuildError::missing_field("zero_data_retention"))?,
        })
    }
}
