pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Pool of phone numbers to use as the caller ID for outbound calls.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigOptionsOutboundNumberPool {
    /// Active E.164 phone numbers to use for outbound calls. Numbers must be unique.
    #[serde(default)]
    pub active: Vec<String>,
    /// Blocked E.164 phone numbers that should not be used for outbound calls.
    #[serde(default)]
    pub blocked: Vec<String>,
}

impl ConfigOptionsOutboundNumberPool {
    pub fn builder() -> ConfigOptionsOutboundNumberPoolBuilder {
        <ConfigOptionsOutboundNumberPoolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigOptionsOutboundNumberPoolBuilder {
    active: Option<Vec<String>>,
    blocked: Option<Vec<String>>,
}

impl ConfigOptionsOutboundNumberPoolBuilder {
    pub fn active(mut self, value: Vec<String>) -> Self {
        self.active = Some(value);
        self
    }

    pub fn blocked(mut self, value: Vec<String>) -> Self {
        self.blocked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigOptionsOutboundNumberPool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active`](ConfigOptionsOutboundNumberPoolBuilder::active)
    /// - [`blocked`](ConfigOptionsOutboundNumberPoolBuilder::blocked)
    pub fn build(self) -> Result<ConfigOptionsOutboundNumberPool, BuildError> {
        Ok(ConfigOptionsOutboundNumberPool {
            active: self.active.ok_or_else(|| BuildError::missing_field("active"))?,
            blocked: self.blocked.ok_or_else(|| BuildError::missing_field("blocked"))?,
        })
    }
}
