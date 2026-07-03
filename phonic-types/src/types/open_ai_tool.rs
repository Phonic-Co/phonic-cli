pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// OpenAI-compatible function tool schema.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenAiTool {
    pub r#type: String,
    pub function: OpenAiFunction,
}

impl OpenAiTool {
    pub fn builder() -> OpenAiToolBuilder {
        <OpenAiToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OpenAiToolBuilder {
    r#type: Option<String>,
    function: Option<OpenAiFunction>,
}

impl OpenAiToolBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn function(mut self, value: OpenAiFunction) -> Self {
        self.function = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OpenAiTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](OpenAiToolBuilder::r#type)
    /// - [`function`](OpenAiToolBuilder::function)
    pub fn build(self) -> Result<OpenAiTool, BuildError> {
        Ok(OpenAiTool {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            function: self.function.ok_or_else(|| BuildError::missing_field("function"))?,
        })
    }
}
