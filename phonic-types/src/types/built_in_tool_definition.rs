pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A built-in tool with an explicit configuration, as an alternative to referencing it by bare name (which uses the tool's default configuration). `keypad_input` and `natural_conversation_ending` take a `speech_before_tool_call` config; `choose_not_to_respond` takes a `respond_after_sec` config.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BuiltInToolDefinition {
    pub r#type: String,
    /// The name of the built-in tool.
    pub name: BuiltInToolDefinitionName,
    /// The tool's configuration. Use `BuiltInToolConfig` for `keypad_input` and `natural_conversation_ending`, or `ChooseNotToRespondToolConfig` for `choose_not_to_respond`.
    pub tool_config: BuiltInToolDefinitionToolConfig,
}

impl BuiltInToolDefinition {
    pub fn builder() -> BuiltInToolDefinitionBuilder {
        <BuiltInToolDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BuiltInToolDefinitionBuilder {
    r#type: Option<String>,
    name: Option<BuiltInToolDefinitionName>,
    tool_config: Option<BuiltInToolDefinitionToolConfig>,
}

impl BuiltInToolDefinitionBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn name(mut self, value: BuiltInToolDefinitionName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn tool_config(mut self, value: BuiltInToolDefinitionToolConfig) -> Self {
        self.tool_config = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BuiltInToolDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](BuiltInToolDefinitionBuilder::r#type)
    /// - [`name`](BuiltInToolDefinitionBuilder::name)
    /// - [`tool_config`](BuiltInToolDefinitionBuilder::tool_config)
    pub fn build(self) -> Result<BuiltInToolDefinition, BuildError> {
        Ok(BuiltInToolDefinition {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tool_config: self.tool_config.ok_or_else(|| BuildError::missing_field("tool_config"))?,
        })
    }
}
