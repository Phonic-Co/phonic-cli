pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Email alert recipients, keyed by alert type. Replaces the workspace's alerts; alert types omitted from this object are removed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateWorkspaceRequestAlerts {
    /// Recipients notified when an agent's configuration endpoint call fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_endpoint_error: Option<WorkspaceAlert>,
}

impl UpdateWorkspaceRequestAlerts {
    pub fn builder() -> UpdateWorkspaceRequestAlertsBuilder {
        <UpdateWorkspaceRequestAlertsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateWorkspaceRequestAlertsBuilder {
    configuration_endpoint_error: Option<WorkspaceAlert>,
}

impl UpdateWorkspaceRequestAlertsBuilder {
    pub fn configuration_endpoint_error(mut self, value: WorkspaceAlert) -> Self {
        self.configuration_endpoint_error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateWorkspaceRequestAlerts`].
    pub fn build(self) -> Result<UpdateWorkspaceRequestAlerts, BuildError> {
        Ok(UpdateWorkspaceRequestAlerts {
            configuration_endpoint_error: self.configuration_endpoint_error,
        })
    }
}
