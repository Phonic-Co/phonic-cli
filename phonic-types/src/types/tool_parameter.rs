pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ToolParameter {
    /// The parameter type.
    pub r#type: ToolParameterType,
    /// Required only when type is "array". The type of items in the array.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<ToolParameterItemType>,
    /// The parameter name.
    #[serde(default)]
    pub name: String,
    /// Description of the parameter.
    #[serde(default)]
    pub description: String,
    /// Whether the parameter is required.
    #[serde(default)]
    pub is_required: bool,
    /// Only applicable for `custom_webhook` tools. Specifies where the parameter should be sent in the webhook request.
    /// - For GET webhooks: defaults to `"query_string"` and `"request_body"` is not allowed.
    /// - For POST webhooks: required, can be either `"request_body"` or `"query_string"`.
    /// - Not allowed for `custom_websocket`, `built_in_transfer_to_phone_number`, or `built_in_transfer_to_agent` tools.
    /// When updating a tool's type or endpoint_method, all parameters must include explicit `location` values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ToolParameterLocation>,
}

impl ToolParameter {
    pub fn builder() -> ToolParameterBuilder {
        <ToolParameterBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolParameterBuilder {
    r#type: Option<ToolParameterType>,
    item_type: Option<ToolParameterItemType>,
    name: Option<String>,
    description: Option<String>,
    is_required: Option<bool>,
    location: Option<ToolParameterLocation>,
}

impl ToolParameterBuilder {
    pub fn r#type(mut self, value: ToolParameterType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn item_type(mut self, value: ToolParameterItemType) -> Self {
        self.item_type = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn is_required(mut self, value: bool) -> Self {
        self.is_required = Some(value);
        self
    }

    pub fn location(mut self, value: ToolParameterLocation) -> Self {
        self.location = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolParameter`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ToolParameterBuilder::r#type)
    /// - [`name`](ToolParameterBuilder::name)
    /// - [`description`](ToolParameterBuilder::description)
    /// - [`is_required`](ToolParameterBuilder::is_required)
    pub fn build(self) -> Result<ToolParameter, BuildError> {
        Ok(ToolParameter {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            item_type: self.item_type,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            is_required: self.is_required.ok_or_else(|| BuildError::missing_field("is_required"))?,
            location: self.location,
        })
    }
}
