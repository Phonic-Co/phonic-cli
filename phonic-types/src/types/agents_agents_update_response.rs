pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentsUpdateResponse {
    /// Whether the agent was updated successfully.
    #[serde(default)]
    pub success: bool,
    /// The version number of the agent after the update.
    #[serde(default)]
    pub version_number: i64,
    /// The updated agent.
    pub agent: Agent,
}

impl AgentsUpdateResponse {
    pub fn builder() -> AgentsUpdateResponseBuilder {
        <AgentsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsUpdateResponseBuilder {
    success: Option<bool>,
    version_number: Option<i64>,
    agent: Option<Agent>,
}

impl AgentsUpdateResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn version_number(mut self, value: i64) -> Self {
        self.version_number = Some(value);
        self
    }

    pub fn agent(mut self, value: Agent) -> Self {
        self.agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](AgentsUpdateResponseBuilder::success)
    /// - [`version_number`](AgentsUpdateResponseBuilder::version_number)
    /// - [`agent`](AgentsUpdateResponseBuilder::agent)
    pub fn build(self) -> Result<AgentsUpdateResponse, BuildError> {
        Ok(AgentsUpdateResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
            version_number: self.version_number.ok_or_else(|| BuildError::missing_field("version_number"))?,
            agent: self.agent.ok_or_else(|| BuildError::missing_field("agent"))?,
        })
    }
}
