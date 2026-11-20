pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// JSON Schema describing the tool's arguments.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct McpServerToolParameters {
    pub r#type: String,
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub required: Vec<String>,
    #[serde(rename = "additionalProperties")]
    pub additional_properties: bool,
}

impl McpServerToolParameters {
    pub fn builder() -> McpServerToolParametersBuilder {
        <McpServerToolParametersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct McpServerToolParametersBuilder {
    r#type: Option<String>,
    properties: Option<HashMap<String, serde_json::Value>>,
    required: Option<Vec<String>>,
    additional_properties: Option<bool>,
}

impl McpServerToolParametersBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn properties(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.properties = Some(value);
        self
    }

    pub fn required(mut self, value: Vec<String>) -> Self {
        self.required = Some(value);
        self
    }

    pub fn additional_properties(mut self, value: bool) -> Self {
        self.additional_properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`McpServerToolParameters`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](McpServerToolParametersBuilder::r#type)
    /// - [`properties`](McpServerToolParametersBuilder::properties)
    /// - [`required`](McpServerToolParametersBuilder::required)
    /// - [`additional_properties`](McpServerToolParametersBuilder::additional_properties)
    pub fn build(self) -> Result<McpServerToolParameters, BuildError> {
        Ok(McpServerToolParameters {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            properties: self.properties.ok_or_else(|| BuildError::missing_field("properties"))?,
            required: self.required.ok_or_else(|| BuildError::missing_field("required"))?,
            additional_properties: self.additional_properties.ok_or_else(|| BuildError::missing_field("additional_properties"))?,
        })
    }
}
