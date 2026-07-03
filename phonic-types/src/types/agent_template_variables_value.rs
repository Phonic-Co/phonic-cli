pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentTemplateVariablesValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

impl AgentTemplateVariablesValue {
    pub fn builder() -> AgentTemplateVariablesValueBuilder {
        <AgentTemplateVariablesValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentTemplateVariablesValueBuilder {
    default_value: Option<String>,
}

impl AgentTemplateVariablesValueBuilder {
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentTemplateVariablesValue`].
    pub fn build(self) -> Result<AgentTemplateVariablesValue, BuildError> {
        Ok(AgentTemplateVariablesValue {
            default_value: self.default_value,
        })
    }
}
