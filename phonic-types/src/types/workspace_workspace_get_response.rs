pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceGetResponse {
    /// Number of conversations currently in progress for this workspace.
    #[serde(default)]
    pub active_conversations: i64,
    /// Maximum number of concurrent conversations allowed for this workspace.
    #[serde(default)]
    pub max_active_conversations: i64,
    /// URL of the workspace logo (PNG or SVG), or null if not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Email domains allowed to join the workspace via invite link.
    #[serde(default)]
    pub invite_link_allowed_domains: Vec<String>,
    /// IP addresses or CIDR ranges allowed to access the workspace.
    #[serde(default)]
    pub ip_allowlist: Vec<String>,
}

impl WorkspaceGetResponse {
    pub fn builder() -> WorkspaceGetResponseBuilder {
        <WorkspaceGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceGetResponseBuilder {
    active_conversations: Option<i64>,
    max_active_conversations: Option<i64>,
    logo_url: Option<String>,
    invite_link_allowed_domains: Option<Vec<String>>,
    ip_allowlist: Option<Vec<String>>,
}

impl WorkspaceGetResponseBuilder {
    pub fn active_conversations(mut self, value: i64) -> Self {
        self.active_conversations = Some(value);
        self
    }

    pub fn max_active_conversations(mut self, value: i64) -> Self {
        self.max_active_conversations = Some(value);
        self
    }

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

    /// Consumes the builder and constructs a [`WorkspaceGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active_conversations`](WorkspaceGetResponseBuilder::active_conversations)
    /// - [`max_active_conversations`](WorkspaceGetResponseBuilder::max_active_conversations)
    /// - [`invite_link_allowed_domains`](WorkspaceGetResponseBuilder::invite_link_allowed_domains)
    /// - [`ip_allowlist`](WorkspaceGetResponseBuilder::ip_allowlist)
    pub fn build(self) -> Result<WorkspaceGetResponse, BuildError> {
        Ok(WorkspaceGetResponse {
            active_conversations: self.active_conversations.ok_or_else(|| BuildError::missing_field("active_conversations"))?,
            max_active_conversations: self.max_active_conversations.ok_or_else(|| BuildError::missing_field("max_active_conversations"))?,
            logo_url: self.logo_url,
            invite_link_allowed_domains: self.invite_link_allowed_domains.ok_or_else(|| BuildError::missing_field("invite_link_allowed_domains"))?,
            ip_allowlist: self.ip_allowlist.ok_or_else(|| BuildError::missing_field("ip_allowlist"))?,
        })
    }
}
