pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateMcpServerRequest {
    /// The name of the MCP server. Must be snake_case, start with a lowercase letter and be unique within the project.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Headers sent on every request to the MCP server. Replaces all existing headers. Header names must be unique (case-insensitive) and values cannot contain line breaks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, String>>,
    /// The name of the project containing the MCP server. Only used when `nameOrId` is a name.
    #[serde(skip)]
    pub project: Option<String>,
}

impl UpdateMcpServerRequest {
    pub fn builder() -> UpdateMcpServerRequestBuilder {
        <UpdateMcpServerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMcpServerRequestBuilder {
    name: Option<String>,
    headers: Option<HashMap<String, String>>,
    project: Option<String>,
}

impl UpdateMcpServerRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    /// Consumes the builder and constructs a [`UpdateMcpServerRequest`].
    pub fn build(self) -> Result<UpdateMcpServerRequest, BuildError> {
        Ok(UpdateMcpServerRequest {
            name: self.name,
            headers: self.headers,
            project: self.project,
        })
    }
}

