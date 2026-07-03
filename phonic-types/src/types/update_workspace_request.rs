pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWorkspaceRequest {
    /// URL of the workspace logo. Must be an https URL ending in .png or .svg, or null to clear it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Email domains allowed to join the workspace via invite link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link_allowed_domains: Option<Vec<String>>,
    /// IP addresses or CIDR ranges allowed to access the workspace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_allowlist: Option<Vec<String>>,
}

impl UpdateWorkspaceRequest {
    pub fn builder() -> UpdateWorkspaceRequestBuilder {
        <UpdateWorkspaceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWorkspaceRequestBuilder {
    logo_url: Option<String>,
    invite_link_allowed_domains: Option<Vec<String>>,
    ip_allowlist: Option<Vec<String>>,
}

impl UpdateWorkspaceRequestBuilder {
    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn invite_link_allowed_domains(mut self, value: Vec<String>) -> Self {
        self.invite_link_allowed_domains = Some(value);
        self
    }

    pub fn ip_allowlist(mut self, value: Vec<String>) -> Self {
        self.ip_allowlist = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateWorkspaceRequest`].
    pub fn build(self) -> Result<UpdateWorkspaceRequest, BuildError> {
        Ok(UpdateWorkspaceRequest {
            logo_url: self.logo_url,
            invite_link_allowed_domains: self.invite_link_allowed_domains,
            ip_allowlist: self.ip_allowlist,
        })
    }
}

