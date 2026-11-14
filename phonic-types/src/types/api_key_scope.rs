pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What an API key can reach. A key can grant another key only a scope
/// within its own: it cannot turn on workspace access it does not have,
/// grant access to every project when its own access is limited to
/// selected projects, or include a project outside its own list.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApiKeyScope {
    /// When true, the key can read and update the workspace, create
    /// projects, and create, update, rotate, and delete API keys within
    /// its own scope. A key can still read and update the projects in its
    /// list when this is false.
    #[serde(default)]
    pub workspace_access: bool,
    /// `all` can use every current and future project in the workspace.
    /// `selected` can use only the projects in `project_ids`.
    pub project_access: ApiKeyScopeProjectAccess,
    /// IDs of the projects this key can use. Required when
    /// `project_access` is `selected`, and must be omitted when it is
    /// `all`. An empty array grants access to no projects. Each ID must
    /// belong to the workspace. Duplicate IDs are removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_ids: Option<Vec<String>>,
}

impl ApiKeyScope {
    pub fn builder() -> ApiKeyScopeBuilder {
        <ApiKeyScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeyScopeBuilder {
    workspace_access: Option<bool>,
    project_access: Option<ApiKeyScopeProjectAccess>,
    project_ids: Option<Vec<String>>,
}

impl ApiKeyScopeBuilder {
    pub fn workspace_access(mut self, value: bool) -> Self {
        self.workspace_access = Some(value);
        self
    }

    pub fn project_access(mut self, value: ApiKeyScopeProjectAccess) -> Self {
        self.project_access = Some(value);
        self
    }

    pub fn project_ids(mut self, value: Vec<String>) -> Self {
        self.project_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeyScope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`workspace_access`](ApiKeyScopeBuilder::workspace_access)
    /// - [`project_access`](ApiKeyScopeBuilder::project_access)
    pub fn build(self) -> Result<ApiKeyScope, BuildError> {
        Ok(ApiKeyScope {
            workspace_access: self.workspace_access.ok_or_else(|| BuildError::missing_field("workspace_access"))?,
            project_access: self.project_access.ok_or_else(|| BuildError::missing_field("project_access"))?,
            project_ids: self.project_ids,
        })
    }
}
