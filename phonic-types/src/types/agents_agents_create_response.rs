pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsCreateResponse {
    /// The ID of the created agent.
    #[serde(default)]
    pub id: String,
    /// The name of the created agent.
    #[serde(default)]
    pub name: String,
}

impl AgentsCreateResponse {
    pub fn builder() -> AgentsCreateResponseBuilder {
        <AgentsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsCreateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl AgentsCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AgentsCreateResponseBuilder::id)
    /// - [`name`](AgentsCreateResponseBuilder::name)
    pub fn build(self) -> Result<AgentsCreateResponse, BuildError> {
        Ok(AgentsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
