pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsUpdatePhoneNumberResponse {
    #[serde(default)]
    pub success: bool,
}

impl AgentsUpdatePhoneNumberResponse {
    pub fn builder() -> AgentsUpdatePhoneNumberResponseBuilder {
        <AgentsUpdatePhoneNumberResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsUpdatePhoneNumberResponseBuilder {
    success: Option<bool>,
}

impl AgentsUpdatePhoneNumberResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsUpdatePhoneNumberResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](AgentsUpdatePhoneNumberResponseBuilder::success)
    pub fn build(self) -> Result<AgentsUpdatePhoneNumberResponse, BuildError> {
        Ok(AgentsUpdatePhoneNumberResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
