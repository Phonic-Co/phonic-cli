pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Configuration for the `choose_not_to_respond` built-in tool.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(transparent)]
pub struct ChooseNotToRespondToolConfig {
    /// Number of seconds to wait after the tool fires before the assistant speaks a follow-up if the user stays silent. When null, the assistant stays silent (default).
    pub respond_after_sec: Option<f64>,
}

impl ChooseNotToRespondToolConfig {
    pub fn builder() -> ChooseNotToRespondToolConfigBuilder {
        <ChooseNotToRespondToolConfigBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChooseNotToRespondToolConfigBuilder {
    respond_after_sec: Option<f64>,
}

impl ChooseNotToRespondToolConfigBuilder {
    pub fn respond_after_sec(mut self, value: f64) -> Self {
        self.respond_after_sec = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChooseNotToRespondToolConfig`].
    pub fn build(self) -> Result<ChooseNotToRespondToolConfig, BuildError> {
        Ok(ChooseNotToRespondToolConfig {
            respond_after_sec: self.respond_after_sec,
        })
    }
}
