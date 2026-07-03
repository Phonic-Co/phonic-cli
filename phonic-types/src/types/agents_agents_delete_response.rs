pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsDeleteResponse {
    /// Whether the agent was deleted successfully.
    #[serde(default)]
    pub success: bool,
}

impl AgentsDeleteResponse {
    pub fn builder() -> AgentsDeleteResponseBuilder {
        <AgentsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsDeleteResponseBuilder {
    success: Option<bool>,
}

impl AgentsDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](AgentsDeleteResponseBuilder::success)
    pub fn build(self) -> Result<AgentsDeleteResponse, BuildError> {
        Ok(AgentsDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
