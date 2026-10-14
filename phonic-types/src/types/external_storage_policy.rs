pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// An S3-compatible destination that conversation artifacts are delivered to.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExternalStoragePolicy {
    /// The ID of the external storage policy.
    #[serde(default)]
    pub id: String,
    /// The name of the external storage policy. Agents reference the policy by this name.
    #[serde(default)]
    pub name: String,
    /// The S3-compatible endpoint URL that artifacts are uploaded to.
    #[serde(default)]
    pub endpoint_url: String,
    /// The bucket that artifacts are uploaded to.
    #[serde(default)]
    pub bucket: String,
    /// The region of the bucket. `null` when the endpoint doesn't require a region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Prefix prepended to every object key. `null` when objects are written at the bucket root.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    /// How bucket names are addressed in requests to the endpoint.
    pub addressing_style: ExternalStoragePolicyAddressingStyle,
    /// Number of agents currently referencing this external storage policy.
    #[serde(default)]
    pub agent_count: i64,
}

impl ExternalStoragePolicy {
    pub fn builder() -> ExternalStoragePolicyBuilder {
        <ExternalStoragePolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExternalStoragePolicyBuilder {
    id: Option<String>,
    name: Option<String>,
    endpoint_url: Option<String>,
    bucket: Option<String>,
    region: Option<String>,
    key_prefix: Option<String>,
    addressing_style: Option<ExternalStoragePolicyAddressingStyle>,
    agent_count: Option<i64>,
}

impl ExternalStoragePolicyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn endpoint_url(mut self, value: impl Into<String>) -> Self {
        self.endpoint_url = Some(value.into());
        self
    }

    pub fn bucket(mut self, value: impl Into<String>) -> Self {
        self.bucket = Some(value.into());
        self
    }

    pub fn region(mut self, value: impl Into<String>) -> Self {
        self.region = Some(value.into());
        self
    }

    pub fn key_prefix(mut self, value: impl Into<String>) -> Self {
        self.key_prefix = Some(value.into());
        self
    }

    pub fn addressing_style(mut self, value: ExternalStoragePolicyAddressingStyle) -> Self {
        self.addressing_style = Some(value);
        self
    }

    pub fn agent_count(mut self, value: i64) -> Self {
        self.agent_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExternalStoragePolicy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ExternalStoragePolicyBuilder::id)
    /// - [`name`](ExternalStoragePolicyBuilder::name)
    /// - [`endpoint_url`](ExternalStoragePolicyBuilder::endpoint_url)
    /// - [`bucket`](ExternalStoragePolicyBuilder::bucket)
    /// - [`addressing_style`](ExternalStoragePolicyBuilder::addressing_style)
    /// - [`agent_count`](ExternalStoragePolicyBuilder::agent_count)
    pub fn build(self) -> Result<ExternalStoragePolicy, BuildError> {
        Ok(ExternalStoragePolicy {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            endpoint_url: self.endpoint_url.ok_or_else(|| BuildError::missing_field("endpoint_url"))?,
            bucket: self.bucket.ok_or_else(|| BuildError::missing_field("bucket"))?,
            region: self.region,
            key_prefix: self.key_prefix,
            addressing_style: self.addressing_style.ok_or_else(|| BuildError::missing_field("addressing_style"))?,
            agent_count: self.agent_count.ok_or_else(|| BuildError::missing_field("agent_count"))?,
        })
    }
}
