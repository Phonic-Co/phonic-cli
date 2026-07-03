pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentsListResponse {
    #[serde(default)]
    pub agents: Vec<Agent>,
}

impl AgentsListResponse {
    pub fn builder() -> AgentsListResponseBuilder {
        <AgentsListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsListResponseBuilder {
    agents: Option<Vec<Agent>>,
}

impl AgentsListResponseBuilder {
    pub fn agents(mut self, value: Vec<Agent>) -> Self {
        self.agents = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agents`](AgentsListResponseBuilder::agents)
    pub fn build(self) -> Result<AgentsListResponse, BuildError> {
        Ok(AgentsListResponse {
            agents: self.agents.ok_or_else(|| BuildError::missing_field("agents"))?,
        })
    }
}
