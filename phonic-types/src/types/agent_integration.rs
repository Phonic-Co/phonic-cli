pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A third-party integration enabled for an agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentIntegration {
    /// The connected account ID.
    #[serde(default)]
    pub account_id: String,
    /// The connected account name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,
    /// The integration actions available to the agent.
    #[serde(default)]
    pub action_names: Vec<String>,
    /// The slug identifying the integration app.
    #[serde(default)]
    pub app_slug: String,
}

impl AgentIntegration {
    pub fn builder() -> AgentIntegrationBuilder {
        <AgentIntegrationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentIntegrationBuilder {
    account_id: Option<String>,
    account_name: Option<String>,
    action_names: Option<Vec<String>>,
    app_slug: Option<String>,
}

impl AgentIntegrationBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn account_name(mut self, value: impl Into<String>) -> Self {
        self.account_name = Some(value.into());
        self
    }

    pub fn action_names(mut self, value: Vec<String>) -> Self {
        self.action_names = Some(value);
        self
    }

    pub fn app_slug(mut self, value: impl Into<String>) -> Self {
        self.app_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentIntegration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](AgentIntegrationBuilder::account_id)
    /// - [`action_names`](AgentIntegrationBuilder::action_names)
    /// - [`app_slug`](AgentIntegrationBuilder::app_slug)
    pub fn build(self) -> Result<AgentIntegration, BuildError> {
        Ok(AgentIntegration {
            account_id: self.account_id.ok_or_else(|| BuildError::missing_field("account_id"))?,
            account_name: self.account_name,
            action_names: self.action_names.ok_or_else(|| BuildError::missing_field("action_names"))?,
            app_slug: self.app_slug.ok_or_else(|| BuildError::missing_field("app_slug"))?,
        })
    }
}
