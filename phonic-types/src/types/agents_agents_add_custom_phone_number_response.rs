pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsAddCustomPhoneNumberResponse {
    #[serde(default)]
    pub success: bool,
}

impl AgentsAddCustomPhoneNumberResponse {
    pub fn builder() -> AgentsAddCustomPhoneNumberResponseBuilder {
        <AgentsAddCustomPhoneNumberResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsAddCustomPhoneNumberResponseBuilder {
    success: Option<bool>,
}

impl AgentsAddCustomPhoneNumberResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsAddCustomPhoneNumberResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](AgentsAddCustomPhoneNumberResponseBuilder::success)
    pub fn build(self) -> Result<AgentsAddCustomPhoneNumberResponse, BuildError> {
        Ok(AgentsAddCustomPhoneNumberResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
