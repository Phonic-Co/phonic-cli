pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateSystemPromptPayload {
    pub r#type: String,
    /// New system prompt (cannot contain template variables)
    #[serde(default)]
    pub system_prompt: String,
}

impl UpdateSystemPromptPayload {
    pub fn builder() -> UpdateSystemPromptPayloadBuilder {
        <UpdateSystemPromptPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSystemPromptPayloadBuilder {
    r#type: Option<String>,
    system_prompt: Option<String>,
}

impl UpdateSystemPromptPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateSystemPromptPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](UpdateSystemPromptPayloadBuilder::r#type)
    /// - [`system_prompt`](UpdateSystemPromptPayloadBuilder::system_prompt)
    pub fn build(self) -> Result<UpdateSystemPromptPayload, BuildError> {
        Ok(UpdateSystemPromptPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            system_prompt: self.system_prompt.ok_or_else(|| BuildError::missing_field("system_prompt"))?,
        })
    }
}
