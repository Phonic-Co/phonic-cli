pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceAlert {
    /// Email addresses that receive the alert. Must be unique. An empty list stores the alert with no recipients.
    #[serde(default)]
    pub emails: Vec<String>,
}

impl WorkspaceAlert {
    pub fn builder() -> WorkspaceAlertBuilder {
        <WorkspaceAlertBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceAlertBuilder {
    emails: Option<Vec<String>>,
}

impl WorkspaceAlertBuilder {
    pub fn emails(mut self, value: Vec<String>) -> Self {
        self.emails = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceAlert`].
    /// This method will fail if any of the following fields are not set:
    /// - [`emails`](WorkspaceAlertBuilder::emails)
    pub fn build(self) -> Result<WorkspaceAlert, BuildError> {
        Ok(WorkspaceAlert {
            emails: self.emails.ok_or_else(|| BuildError::missing_field("emails"))?,
        })
    }
}
