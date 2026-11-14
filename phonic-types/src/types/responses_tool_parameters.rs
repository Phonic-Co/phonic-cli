pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A JSON Schema object describing the tool's parameters, accepted on the same terms as an inline `custom_websocket` tool's parameters. The top level must be an object with `properties`, and the whole schema must be valid JSON Schema.
/// Parameter names cannot be any of the reserved names that Phonic injects into every tool call: `call_info`, `conversation_id`, `from_phone_number`, `pre_tool_speech`, `pre_tool_text`, `to_phone_number`, `twilio_call_sid`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponsesToolParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// The tool's top-level parameters, as a map from parameter name to its JSON Schema.
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ResponsesToolParameters {
    pub fn builder() -> ResponsesToolParametersBuilder {
        <ResponsesToolParametersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesToolParametersBuilder {
    r#type: Option<String>,
    properties: Option<HashMap<String, serde_json::Value>>,
}

impl ResponsesToolParametersBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn properties(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.properties = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponsesToolParameters`].
    /// This method will fail if any of the following fields are not set:
    /// - [`properties`](ResponsesToolParametersBuilder::properties)
    pub fn build(self) -> Result<ResponsesToolParameters, BuildError> {
        Ok(ResponsesToolParameters {
            r#type: self.r#type,
            properties: self.properties.ok_or_else(|| BuildError::missing_field("properties"))?,
            extra: Default::default(),
        })
    }
}
