pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentsUpdatePhoneNumberRequest {
    /// The E.164 formatted phone number to add (e.g., "+15551234567").
    #[serde(default)]
    pub phone_number: String,
    /// When not `null`, the agent will call this endpoint to get configuration options for calls on this phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_endpoint: Option<AgentsUpdatePhoneNumberRequestConfigurationEndpoint>,
    /// The name of the project containing the agent. Only used when `nameOrId` is a name.
    #[serde(skip)]
    pub project: Option<String>,
}

impl AgentsUpdatePhoneNumberRequest {
    pub fn builder() -> AgentsUpdatePhoneNumberRequestBuilder {
        <AgentsUpdatePhoneNumberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsUpdatePhoneNumberRequestBuilder {
    phone_number: Option<String>,
    configuration_endpoint: Option<AgentsUpdatePhoneNumberRequestConfigurationEndpoint>,
    project: Option<String>,
}

impl AgentsUpdatePhoneNumberRequestBuilder {
    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn configuration_endpoint(mut self, value: AgentsUpdatePhoneNumberRequestConfigurationEndpoint) -> Self {
        self.configuration_endpoint = Some(value);
        self
    }

    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentsUpdatePhoneNumberRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`phone_number`](AgentsUpdatePhoneNumberRequestBuilder::phone_number)
    pub fn build(self) -> Result<AgentsUpdatePhoneNumberRequest, BuildError> {
        Ok(AgentsUpdatePhoneNumberRequest {
            phone_number: self.phone_number.ok_or_else(|| BuildError::missing_field("phone_number"))?,
            configuration_endpoint: self.configuration_endpoint,
            project: self.project,
        })
    }
}

