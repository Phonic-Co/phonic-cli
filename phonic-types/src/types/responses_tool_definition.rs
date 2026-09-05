pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A tool the assistant may call, defined inline for this request only.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResponsesToolDefinition {
    /// The tool name.
    #[serde(default)]
    pub name: String,
    /// What the tool does. The model uses this to decide when to call it.
    #[serde(default)]
    pub description: String,
    pub parameters: ToolParametersJsonSchema,
}

impl ResponsesToolDefinition {
    pub fn builder() -> ResponsesToolDefinitionBuilder {
        <ResponsesToolDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesToolDefinitionBuilder {
    name: Option<String>,
    description: Option<String>,
    parameters: Option<ToolParametersJsonSchema>,
}

impl ResponsesToolDefinitionBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn parameters(mut self, value: ToolParametersJsonSchema) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesToolDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ResponsesToolDefinitionBuilder::name)
    /// - [`description`](ResponsesToolDefinitionBuilder::description)
    /// - [`parameters`](ResponsesToolDefinitionBuilder::parameters)
    pub fn build(self) -> Result<ResponsesToolDefinition, BuildError> {
        Ok(ResponsesToolDefinition {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            parameters: self.parameters.ok_or_else(|| BuildError::missing_field("parameters"))?,
        })
    }
}
