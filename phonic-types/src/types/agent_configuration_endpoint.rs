pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// When not `null`, the agent will call this endpoint to get configuration options.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentConfigurationEndpoint {
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// The timeout for the configuration endpoint in milliseconds.
    #[serde(default)]
    pub timeout_ms: i64,
}

impl AgentConfigurationEndpoint {
    pub fn builder() -> AgentConfigurationEndpointBuilder {
        <AgentConfigurationEndpointBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentConfigurationEndpointBuilder {
    url: Option<String>,
    headers: Option<HashMap<String, String>>,
    timeout_ms: Option<i64>,
}

impl AgentConfigurationEndpointBuilder {
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

    /// Consumes the builder and constructs a [`AgentConfigurationEndpoint`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](AgentConfigurationEndpointBuilder::url)
    /// - [`headers`](AgentConfigurationEndpointBuilder::headers)
    /// - [`timeout_ms`](AgentConfigurationEndpointBuilder::timeout_ms)
    pub fn build(self) -> Result<AgentConfigurationEndpoint, BuildError> {
        Ok(AgentConfigurationEndpoint {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            headers: self.headers.ok_or_else(|| BuildError::missing_field("headers"))?,
            timeout_ms: self.timeout_ms.ok_or_else(|| BuildError::missing_field("timeout_ms"))?,
        })
    }
}
