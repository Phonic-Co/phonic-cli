pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentsDeleteCustomPhoneNumberRequest {
    /// The E.164 formatted phone number to remove (e.g., "+15551234567").
    #[serde(default)]
    pub phone_number: String,
    /// The name of the project containing the agent. Only used when `nameOrId` is a name.
    #[serde(skip)]
    pub project: Option<String>,
}

impl AgentsDeleteCustomPhoneNumberRequest {
    pub fn builder() -> AgentsDeleteCustomPhoneNumberRequestBuilder {
        <AgentsDeleteCustomPhoneNumberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsDeleteCustomPhoneNumberRequestBuilder {
    phone_number: Option<String>,
    project: Option<String>,
}

impl AgentsDeleteCustomPhoneNumberRequestBuilder {
    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsDeleteCustomPhoneNumberRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number`](AgentsDeleteCustomPhoneNumberRequestBuilder::phone_number)
    pub fn build(self) -> Result<AgentsDeleteCustomPhoneNumberRequest, BuildError> {
        Ok(AgentsDeleteCustomPhoneNumberRequest {
            phone_number: self.phone_number.ok_or_else(|| BuildError::missing_field("phone_number"))?,
            project: self.project,
        })
    }
}

