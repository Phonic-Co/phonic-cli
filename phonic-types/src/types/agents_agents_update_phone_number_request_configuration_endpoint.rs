pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// When not `null`, the agent will call this endpoint to get configuration options for calls on this phone number.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentsUpdatePhoneNumberRequestConfigurationEndpoint {
    /// URL to call
    #[serde(default)]
    pub url: String,
    /// Object of key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// Timeout in milliseconds for the endpoint call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_ms: Option<i64>,
}

impl AgentsUpdatePhoneNumberRequestConfigurationEndpoint {
    pub fn builder() -> AgentsUpdatePhoneNumberRequestConfigurationEndpointBuilder {
        <AgentsUpdatePhoneNumberRequestConfigurationEndpointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentsUpdatePhoneNumberRequestConfigurationEndpointBuilder {
    url: Option<String>,
    headers: Option<HashMap<String, String>>,
    timeout_ms: Option<i64>,
}

impl AgentsUpdatePhoneNumberRequestConfigurationEndpointBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn headers(mut self, value: HashMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn timeout_ms(mut self, value: i64) -> Self {
        self.timeout_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentsUpdatePhoneNumberRequestConfigurationEndpoint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](AgentsUpdatePhoneNumberRequestConfigurationEndpointBuilder::url)
    pub fn build(self) -> Result<AgentsUpdatePhoneNumberRequestConfigurationEndpoint, BuildError> {
        Ok(AgentsUpdatePhoneNumberRequestConfigurationEndpoint {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            headers: self.headers,
            timeout_ms: self.timeout_ms,
        })
    }
}
