pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Email alert recipients for the workspace, keyed by alert type.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceGetResponseAlerts {
    /// Recipients notified when an agent's configuration endpoint call fails.
    #[serde(default)]
    pub configuration_endpoint_error: WorkspaceAlert,
}

impl WorkspaceGetResponseAlerts {
    pub fn builder() -> WorkspaceGetResponseAlertsBuilder {
        <WorkspaceGetResponseAlertsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceGetResponseAlertsBuilder {
    configuration_endpoint_error: Option<WorkspaceAlert>,
}

impl WorkspaceGetResponseAlertsBuilder {
    pub fn configuration_endpoint_error(mut self, value: WorkspaceAlert) -> Self {
        self.configuration_endpoint_error = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceGetResponseAlerts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`configuration_endpoint_error`](WorkspaceGetResponseAlertsBuilder::configuration_endpoint_error)
    pub fn build(self) -> Result<WorkspaceGetResponseAlerts, BuildError> {
        Ok(WorkspaceGetResponseAlerts {
            configuration_endpoint_error: self.configuration_endpoint_error.ok_or_else(|| BuildError::missing_field("configuration_endpoint_error"))?,
        })
    }
}
