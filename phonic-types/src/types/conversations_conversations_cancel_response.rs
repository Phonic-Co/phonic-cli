pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsCancelResponse {
    /// Whether the conversation was cancelled successfully.
    #[serde(default)]
    pub success: bool,
}

impl ConversationsCancelResponse {
    pub fn builder() -> ConversationsCancelResponseBuilder {
        <ConversationsCancelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsCancelResponseBuilder {
    success: Option<bool>,
}

impl ConversationsCancelResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsCancelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ConversationsCancelResponseBuilder::success)
    pub fn build(self) -> Result<ConversationsCancelResponse, BuildError> {
        Ok(ConversationsCancelResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
