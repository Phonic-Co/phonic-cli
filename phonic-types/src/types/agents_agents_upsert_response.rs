pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentsUpsertResponse {
    pub agent: Agent,
    /// Whether the agent was inserted.
    #[serde(default)]
    pub inserted: bool,
    /// Whether the agent was updated.
    #[serde(default)]
    pub updated: bool,
}

impl AgentsUpsertResponse {
    pub fn builder() -> AgentsUpsertResponseBuilder {
        <AgentsUpsertResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsUpsertResponseBuilder {
    agent: Option<Agent>,
    inserted: Option<bool>,
    updated: Option<bool>,
}

impl AgentsUpsertResponseBuilder {
    pub fn agent(mut self, value: Agent) -> Self {
        self.agent = Some(value);
        self
    }

    pub fn inserted(mut self, value: bool) -> Self {
        self.inserted = Some(value);
        self
    }

    pub fn updated(mut self, value: bool) -> Self {
        self.updated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsUpsertResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent`](AgentsUpsertResponseBuilder::agent)
    /// - [`inserted`](AgentsUpsertResponseBuilder::inserted)
    /// - [`updated`](AgentsUpsertResponseBuilder::updated)
    pub fn build(self) -> Result<AgentsUpsertResponse, BuildError> {
        Ok(AgentsUpsertResponse {
            agent: self.agent.ok_or_else(|| BuildError::missing_field("agent"))?,
            inserted: self.inserted.ok_or_else(|| BuildError::missing_field("inserted"))?,
            updated: self.updated.ok_or_else(|| BuildError::missing_field("updated"))?,
        })
    }
}
