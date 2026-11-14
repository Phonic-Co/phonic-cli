pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A template variable saved on the agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAgentRequestTemplateVariablesValue {
    /// Value used when a conversation doesn't provide one. Can be an empty string.
    #[serde(default)]
    pub default_value: String,
    /// When true, the variable's value is added to the agent's boosted keywords. Boosted values can be at most 50 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_boosted_keyword: Option<bool>,
}

impl CreateAgentRequestTemplateVariablesValue {
    pub fn builder() -> CreateAgentRequestTemplateVariablesValueBuilder {
        <CreateAgentRequestTemplateVariablesValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAgentRequestTemplateVariablesValueBuilder {
    default_value: Option<String>,
    is_boosted_keyword: Option<bool>,
}

impl CreateAgentRequestTemplateVariablesValueBuilder {
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn is_boosted_keyword(mut self, value: bool) -> Self {
        self.is_boosted_keyword = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAgentRequestTemplateVariablesValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`default_value`](CreateAgentRequestTemplateVariablesValueBuilder::default_value)
    pub fn build(self) -> Result<CreateAgentRequestTemplateVariablesValue, BuildError> {
        Ok(CreateAgentRequestTemplateVariablesValue {
            default_value: self.default_value.ok_or_else(|| BuildError::missing_field("default_value"))?,
            is_boosted_keyword: self.is_boosted_keyword,
        })
    }
}
