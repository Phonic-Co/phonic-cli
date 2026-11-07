pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateMcpServerRequest {
    /// The name of the MCP server. Must be snake_case, start with a lowercase letter and be unique within the project.
    #[serde(default)]
    pub name: String,
    /// The URL of the MCP server. Must be a publicly routable HTTPS URL without embedded credentials. Cannot be changed after creation.
    #[serde(default)]
    pub url: String,
    /// Headers sent on every request to the MCP server (e.g. for authentication). Header names must be unique (case-insensitive) and values cannot contain line breaks. Values are stored encrypted and never returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// The name of the project to create the MCP server in.
    #[serde(skip)]
    pub project: Option<String>,
}

impl CreateMcpServerRequest {
    pub fn builder() -> CreateMcpServerRequestBuilder {
        <CreateMcpServerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMcpServerRequestBuilder {
    name: Option<String>,
    url: Option<String>,
    headers: Option<HashMap<String, String>>,
    project: Option<String>,
}

impl CreateMcpServerRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn headers(mut self, value: HashMap<String, String>) -> Self {
        self.headers = Some(value);
        self
    }

    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMcpServerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateMcpServerRequestBuilder::name)
    /// - [`url`](CreateMcpServerRequestBuilder::url)
    pub fn build(self) -> Result<CreateMcpServerRequest, BuildError> {
        Ok(CreateMcpServerRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
            headers: self.headers,
            project: self.project,
        })
    }
}

