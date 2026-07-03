pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAgentRequestTemplateVariablesValue {
    #[serde(default)]
    pub default_value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_boosted_keyword: Option<bool>,
}

impl UpdateAgentRequestTemplateVariablesValue {
    pub fn builder() -> UpdateAgentRequestTemplateVariablesValueBuilder {
        <UpdateAgentRequestTemplateVariablesValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAgentRequestTemplateVariablesValueBuilder {
    default_value: Option<String>,
    is_boosted_keyword: Option<bool>,
}

impl UpdateAgentRequestTemplateVariablesValueBuilder {
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }

    pub fn is_boosted_keyword(mut self, value: bool) -> Self {
        self.is_boosted_keyword = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAgentRequestTemplateVariablesValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`default_value`](UpdateAgentRequestTemplateVariablesValueBuilder::default_value)
    pub fn build(self) -> Result<UpdateAgentRequestTemplateVariablesValue, BuildError> {
        Ok(UpdateAgentRequestTemplateVariablesValue {
            default_value: self.default_value.ok_or_else(|| BuildError::missing_field("default_value"))?,
            is_boosted_keyword: self.is_boosted_keyword,
        })
    }
}
