pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Pool of phone numbers used for outbound calls.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OutboundNumberPool {
    /// E.164 formatted phone numbers available for outbound calls.
    #[serde(default)]
    pub active: Vec<String>,
    /// E.164 formatted phone numbers that are blocked from being used.
    #[serde(default)]
    pub blocked: Vec<String>,
}

impl OutboundNumberPool {
    pub fn builder() -> OutboundNumberPoolBuilder {
        <OutboundNumberPoolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutboundNumberPoolBuilder {
    active: Option<Vec<String>>,
    blocked: Option<Vec<String>>,
}

impl OutboundNumberPoolBuilder {
    pub fn active(mut self, value: Vec<String>) -> Self {
        self.active = Some(value);
        self
    }

    pub fn blocked(mut self, value: Vec<String>) -> Self {
        self.blocked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OutboundNumberPool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active`](OutboundNumberPoolBuilder::active)
    /// - [`blocked`](OutboundNumberPoolBuilder::blocked)
    pub fn build(self) -> Result<OutboundNumberPool, BuildError> {
        Ok(OutboundNumberPool {
            active: self.active.ok_or_else(|| BuildError::missing_field("active"))?,
            blocked: self.blocked.ok_or_else(|| BuildError::missing_field("blocked"))?,
        })
    }
}
