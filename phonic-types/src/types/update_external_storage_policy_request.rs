pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateExternalStoragePolicyRequest {
    /// The name of the external storage policy. Must be snake_case, start with a lowercase letter and be unique within the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The S3-compatible endpoint URL that artifacts are uploaded to. Must be a publicly routable HTTPS URL without embedded credentials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    /// The bucket that artifacts are uploaded to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// The region of the bucket. Set to `null` when the endpoint doesn't require a region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// Prefix prepended to every object key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    /// How bucket names are addressed in requests to the endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addressing_style: Option<UpdateExternalStoragePolicyRequestAddressingStyle>,
    /// Access key ID used to authenticate with the endpoint. Must be provided together with `secret_access_key`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    /// Secret access key used to authenticate with the endpoint. Must be provided together with `access_key_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    /// The name of the project containing the external storage policy. Only used when `nameOrId` is a name.
    #[serde(skip)]
    pub project: Option<String>,
}

impl UpdateExternalStoragePolicyRequest {
    pub fn builder() -> UpdateExternalStoragePolicyRequestBuilder {
        <UpdateExternalStoragePolicyRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateExternalStoragePolicyRequestBuilder {
    name: Option<String>,
    endpoint_url: Option<String>,
    bucket: Option<String>,
    region: Option<String>,
    key_prefix: Option<String>,
    addressing_style: Option<UpdateExternalStoragePolicyRequestAddressingStyle>,
    access_key_id: Option<String>,
    secret_access_key: Option<String>,
    project: Option<String>,
}

impl UpdateExternalStoragePolicyRequestBuilder {
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

    pub fn addressing_style(mut self, value: UpdateExternalStoragePolicyRequestAddressingStyle) -> Self {
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

    /// Consumes the builder and constructs a [`UpdateExternalStoragePolicyRequest`].
    pub fn build(self) -> Result<UpdateExternalStoragePolicyRequest, BuildError> {
        Ok(UpdateExternalStoragePolicyRequest {
            name: self.name,
            endpoint_url: self.endpoint_url,
            bucket: self.bucket,
            region: self.region,
            key_prefix: self.key_prefix,
            addressing_style: self.addressing_style,
            access_key_id: self.access_key_id,
            secret_access_key: self.secret_access_key,
            project: self.project,
        })
    }
}

