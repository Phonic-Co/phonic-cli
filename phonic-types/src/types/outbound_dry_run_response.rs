pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutboundDryRunResponse {
    /// Always null when `dry_run` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<serde_json::Value>,
    /// Always true for this response.
    pub dry_run: bool,
}

impl OutboundDryRunResponse {
    pub fn builder() -> OutboundDryRunResponseBuilder {
        <OutboundDryRunResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OutboundDryRunResponseBuilder {
    conversation_id: Option<serde_json::Value>,
    dry_run: Option<bool>,
}

impl OutboundDryRunResponseBuilder {
    pub fn conversation_id(mut self, value: serde_json::Value) -> Self {
        self.conversation_id = Some(value);
        self
    }

    pub fn dry_run(mut self, value: bool) -> Self {
        self.dry_run = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OutboundDryRunResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`dry_run`](OutboundDryRunResponseBuilder::dry_run)
    pub fn build(self) -> Result<OutboundDryRunResponse, BuildError> {
        Ok(OutboundDryRunResponse {
            conversation_id: self.conversation_id,
            dry_run: self.dry_run.ok_or_else(|| BuildError::missing_field("dry_run"))?,
        })
    }
}
