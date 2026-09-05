pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateExternalStoragePolicyRequest {
    /// The name of the external storage policy. Must be snake_case, start with a lowercase letter and be unique within the project.
    #[serde(default)]
    pub name: String,
    /// The S3-compatible endpoint URL that artifacts are uploaded to. Must be a publicly routable HTTPS URL without embedded credentials.
    #[serde(default)]
    pub endpoint_url: String,
    /// The bucket that artifacts are uploaded to.
    #[serde(default)]
    pub bucket: String,
    /// The region of the bucket. Set to `null` when the endpoint doesn't require a region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Prefix prepended to every object key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    /// How bucket names are addressed in requests to the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addressing_style: Option<CreateExternalStoragePolicyRequestAddressingStyle>,
    /// Access key ID used to authenticate with the endpoint. Stored encrypted and never returned.
    #[serde(default)]
    pub access_key_id: String,
    /// Secret access key used to authenticate with the endpoint. Stored encrypted and never returned.
    #[serde(default)]
    pub secret_access_key: String,
    /// The name of the project to create the external storage policy in.
    #[serde(skip)]
    pub project: Option<String>,
}

impl CreateExternalStoragePolicyRequest {
    pub fn builder() -> CreateExternalStoragePolicyRequestBuilder {
        <CreateExternalStoragePolicyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateExternalStoragePolicyRequestBuilder {
    name: Option<String>,
    endpoint_url: Option<String>,
    bucket: Option<String>,
    region: Option<String>,
    key_prefix: Option<String>,
    addressing_style: Option<CreateExternalStoragePolicyRequestAddressingStyle>,
    access_key_id: Option<String>,
    secret_access_key: Option<String>,
    project: Option<String>,
}

impl CreateExternalStoragePolicyRequestBuilder {
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

    pub fn addressing_style(mut self, value: CreateExternalStoragePolicyRequestAddressingStyle) -> Self {
        self.addressing_style = Some(value);
        self
    }

    pub fn access_key_id(mut self, value: impl Into<String>) -> Self {
        self.access_key_id = Some(value.into());
        self
    }

    pub fn secret_access_key(mut self, value: impl Into<String>) -> Self {
        self.secret_access_key = Some(value.into());
        self
    }

    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateExternalStoragePolicyRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateExternalStoragePolicyRequestBuilder::name)
    /// - [`endpoint_url`](CreateExternalStoragePolicyRequestBuilder::endpoint_url)
    /// - [`bucket`](CreateExternalStoragePolicyRequestBuilder::bucket)
    /// - [`access_key_id`](CreateExternalStoragePolicyRequestBuilder::access_key_id)
    /// - [`secret_access_key`](CreateExternalStoragePolicyRequestBuilder::secret_access_key)
    pub fn build(self) -> Result<CreateExternalStoragePolicyRequest, BuildError> {
        Ok(CreateExternalStoragePolicyRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            endpoint_url: self.endpoint_url.ok_or_else(|| BuildError::missing_field("endpoint_url"))?,
            bucket: self.bucket.ok_or_else(|| BuildError::missing_field("bucket"))?,
            region: self.region,
            key_prefix: self.key_prefix,
            addressing_style: self.addressing_style,
            access_key_id: self.access_key_id.ok_or_else(|| BuildError::missing_field("access_key_id"))?,
            secret_access_key: self.secret_access_key.ok_or_else(|| BuildError::missing_field("secret_access_key"))?,
            project: self.project,
        })
    }
}

