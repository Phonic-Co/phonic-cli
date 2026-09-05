pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A tool's parameters expressed as a raw JSON Schema object, for parameters that the flat `ToolParameter` list cannot express: nested objects, arrays of objects, `anyOf` variants, `null`, and non-string enums.
/// Each entry in `properties` is a JSON Schema value supporting `type` (`"string"`, `"integer"`, `"number"`, `"boolean"`, `"null"`, `"array"`, `"object"`), `description`, `enum` (string parameters only), `items` (for arrays), `properties`/`required`/`additionalProperties` (for objects) and `anyOf`. Values may be nested up to 5 levels deep.
/// Parameter names cannot be any of the reserved names that Phonic injects into every tool call: `call_info`, `conversation_id`, `from_phone_number`, `to_phone_number`, `twilio_call_sid`.
/// For `custom_webhook` tools, parameter placement is supplied separately in `parameter_locations` rather than inline on the schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolParametersJsonSchema {
    pub r#type: String,
    /// The tool's top-level parameters, as a map from parameter name to its JSON Schema.
    #[serde(default)]
    pub properties: HashMap<String, serde_json::Value>,
    /// The names of the required top-level parameters. Every name must be defined in `properties`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    /// Must be `false`. Tool parameter schemas do not allow properties beyond the ones declared.
    #[serde(rename = "additionalProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_properties: Option<bool>,
}

impl ToolParametersJsonSchema {
    pub fn builder() -> ToolParametersJsonSchemaBuilder {
        <ToolParametersJsonSchemaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolParametersJsonSchemaBuilder {
    r#type: Option<String>,
    properties: Option<HashMap<String, serde_json::Value>>,
    required: Option<Vec<String>>,
    additional_properties: Option<bool>,
}

impl ToolParametersJsonSchemaBuilder {
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

    /// Consumes the builder and constructs a [`ToolParametersJsonSchema`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ToolParametersJsonSchemaBuilder::r#type)
    /// - [`properties`](ToolParametersJsonSchemaBuilder::properties)
    pub fn build(self) -> Result<ToolParametersJsonSchema, BuildError> {
        Ok(ToolParametersJsonSchema {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            properties: self.properties.ok_or_else(|| BuildError::missing_field("properties"))?,
            required: self.required,
            additional_properties: self.additional_properties,
        })
    }
}
