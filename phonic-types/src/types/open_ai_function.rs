pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenAiFunction {
    /// Function name the assistant will use in `tool_call.tool_name`.
    #[serde(default)]
    pub name: String,
    /// Description of what the tool does and when to call it.
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub parameters: OpenAiFunctionParameters,
    /// Inline tools require strict function calling.
    pub strict: bool,
}

impl OpenAiFunction {
    pub fn builder() -> OpenAiFunctionBuilder {
        <OpenAiFunctionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiFunctionBuilder {
    name: Option<String>,
    description: Option<String>,
    parameters: Option<OpenAiFunctionParameters>,
    strict: Option<bool>,
}

impl OpenAiFunctionBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn parameters(mut self, value: OpenAiFunctionParameters) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn strict(mut self, value: bool) -> Self {
        self.strict = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiFunction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](OpenAiFunctionBuilder::name)
    /// - [`description`](OpenAiFunctionBuilder::description)
    /// - [`parameters`](OpenAiFunctionBuilder::parameters)
    /// - [`strict`](OpenAiFunctionBuilder::strict)
    pub fn build(self) -> Result<OpenAiFunction, BuildError> {
        Ok(OpenAiFunction {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            parameters: self.parameters.ok_or_else(|| BuildError::missing_field("parameters"))?,
            strict: self.strict.ok_or_else(|| BuildError::missing_field("strict"))?,
        })
    }
}
