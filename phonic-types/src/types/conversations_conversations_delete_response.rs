pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsDeleteResponse {
    /// Whether the conversation was deleted successfully.
    #[serde(default)]
    pub success: bool,
}

impl ConversationsDeleteResponse {
    pub fn builder() -> ConversationsDeleteResponseBuilder {
        <ConversationsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsDeleteResponseBuilder {
    success: Option<bool>,
}

impl ConversationsDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ConversationsDeleteResponseBuilder::success)
    pub fn build(self) -> Result<ConversationsDeleteResponse, BuildError> {
        Ok(ConversationsDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
