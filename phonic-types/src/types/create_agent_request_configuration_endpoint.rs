pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// When not `null`, at the beginning of the conversation the agent will make a POST request to this endpoint to get configuration options.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAgentRequestConfigurationEndpoint {
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

impl CreateAgentRequestConfigurationEndpoint {
    pub fn builder() -> CreateAgentRequestConfigurationEndpointBuilder {
        <CreateAgentRequestConfigurationEndpointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentRequestConfigurationEndpointBuilder {
    url: Option<String>,
    headers: Option<HashMap<String, String>>,
    timeout_ms: Option<i64>,
}

impl CreateAgentRequestConfigurationEndpointBuilder {
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

    /// Consumes the builder and constructs a [`CreateAgentRequestConfigurationEndpoint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](CreateAgentRequestConfigurationEndpointBuilder::url)
    pub fn build(self) -> Result<CreateAgentRequestConfigurationEndpoint, BuildError> {
        Ok(CreateAgentRequestConfigurationEndpoint {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            headers: self.headers,
            timeout_ms: self.timeout_ms,
        })
    }
}
