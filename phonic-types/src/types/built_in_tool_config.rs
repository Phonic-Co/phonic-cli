pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Configuration for a simple built-in tool (`keypad_input`, `natural_conversation_ending`, or `choose_not_to_respond`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BuiltInToolConfig {
    /// Controls whether the assistant speaks before the tool is called. `required`: the assistant must speak first. `optional`: the model decides. `suppressed`: the assistant is strongly instructed to stay silent before the call (best effort).
    pub speech_before_tool_call: BuiltInToolConfigSpeechBeforeToolCall,
}

impl BuiltInToolConfig {
    pub fn builder() -> BuiltInToolConfigBuilder {
        <BuiltInToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BuiltInToolConfigBuilder {
    speech_before_tool_call: Option<BuiltInToolConfigSpeechBeforeToolCall>,
}

impl BuiltInToolConfigBuilder {
    pub fn speech_before_tool_call(mut self, value: BuiltInToolConfigSpeechBeforeToolCall) -> Self {
        self.speech_before_tool_call = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BuiltInToolConfig`].
    /// This method will fail if any of the following fields are not set:
    /// - [`speech_before_tool_call`](BuiltInToolConfigBuilder::speech_before_tool_call)
    pub fn build(self) -> Result<BuiltInToolConfig, BuildError> {
        Ok(BuiltInToolConfig {
            speech_before_tool_call: self.speech_before_tool_call.ok_or_else(|| BuildError::missing_field("speech_before_tool_call"))?,
        })
    }
}
