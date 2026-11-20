pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsEnableResponse {
    #[serde(default)]
    pub success: bool,
}

impl AgentsEnableResponse {
    pub fn builder() -> AgentsEnableResponseBuilder {
        <AgentsEnableResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsEnableResponseBuilder {
    success: Option<bool>,
}

impl AgentsEnableResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsEnableResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](AgentsEnableResponseBuilder::success)
    pub fn build(self) -> Result<AgentsEnableResponse, BuildError> {
        Ok(AgentsEnableResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
