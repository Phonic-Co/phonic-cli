pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsDeleteCustomPhoneNumberResponse {
    #[serde(default)]
    pub success: bool,
}

impl AgentsDeleteCustomPhoneNumberResponse {
    pub fn builder() -> AgentsDeleteCustomPhoneNumberResponseBuilder {
        <AgentsDeleteCustomPhoneNumberResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsDeleteCustomPhoneNumberResponseBuilder {
    success: Option<bool>,
}

impl AgentsDeleteCustomPhoneNumberResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsDeleteCustomPhoneNumberResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](AgentsDeleteCustomPhoneNumberResponseBuilder::success)
    pub fn build(self) -> Result<AgentsDeleteCustomPhoneNumberResponse, BuildError> {
        Ok(AgentsDeleteCustomPhoneNumberResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
