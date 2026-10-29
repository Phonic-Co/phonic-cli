pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Configuration for the `web_search` built-in tool in an agent's `built_in_tool_configs`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct WebSearchToolConfig {
    /// Controls whether the agent speaks before the tool is called. `required`: the agent must speak first. `optional`: the model decides. `suppressed`: the agent is strongly instructed to stay silent before the call (best effort).
    pub speech_before_tool_call: WebSearchToolConfigSpeechBeforeToolCall,
    /// When true, the agent can call web search while the user is still speaking. Setting it to true requires the feature to be enabled for your organization; contact Phonic to get access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_run_during_user_speech: Option<bool>,
}

impl WebSearchToolConfig {
    pub fn builder() -> WebSearchToolConfigBuilder {
        <WebSearchToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebSearchToolConfigBuilder {
    speech_before_tool_call: Option<WebSearchToolConfigSpeechBeforeToolCall>,
    can_run_during_user_speech: Option<bool>,
}

impl WebSearchToolConfigBuilder {
    pub fn speech_before_tool_call(mut self, value: WebSearchToolConfigSpeechBeforeToolCall) -> Self {
        self.speech_before_tool_call = Some(value);
        self
    }

    pub fn can_run_during_user_speech(mut self, value: bool) -> Self {
        self.can_run_during_user_speech = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WebSearchToolConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`speech_before_tool_call`](WebSearchToolConfigBuilder::speech_before_tool_call)
    pub fn build(self) -> Result<WebSearchToolConfig, BuildError> {
        Ok(WebSearchToolConfig {
            speech_before_tool_call: self.speech_before_tool_call.ok_or_else(|| BuildError::missing_field("speech_before_tool_call"))?,
            can_run_during_user_speech: self.can_run_during_user_speech,
        })
    }
}
