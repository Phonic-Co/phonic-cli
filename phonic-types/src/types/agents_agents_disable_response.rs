pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsDisableResponse {
    #[serde(default)]
    pub success: bool,
}

impl AgentsDisableResponse {
    pub fn builder() -> AgentsDisableResponseBuilder {
        <AgentsDisableResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsDisableResponseBuilder {
    success: Option<bool>,
}

impl AgentsDisableResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsDisableResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](AgentsDisableResponseBuilder::success)
    pub fn build(self) -> Result<AgentsDisableResponse, BuildError> {
        Ok(AgentsDisableResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
