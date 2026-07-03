pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentsGetResponse {
    pub agent: Agent,
}

impl AgentsGetResponse {
    pub fn builder() -> AgentsGetResponseBuilder {
        <AgentsGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsGetResponseBuilder {
    agent: Option<Agent>,
}

impl AgentsGetResponseBuilder {
    pub fn agent(mut self, value: Agent) -> Self {
        self.agent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent`](AgentsGetResponseBuilder::agent)
    pub fn build(self) -> Result<AgentsGetResponse, BuildError> {
        Ok(AgentsGetResponse {
            agent: self.agent.ok_or_else(|| BuildError::missing_field("agent"))?,
        })
    }
}
