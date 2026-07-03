pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsListResponseConversationsPagination {
    /// Cursor to fetch the previous page of conversations (newer). Use this value in the `before` query parameter. `null` if there is no previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev_cursor: Option<String>,
    /// Cursor to fetch the next page of conversations (older). Use this value in the `after` query parameter. `null` if there is no next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl ConversationsListResponseConversationsPagination {
    pub fn builder() -> ConversationsListResponseConversationsPaginationBuilder {
        <ConversationsListResponseConversationsPaginationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsListResponseConversationsPaginationBuilder {
    prev_cursor: Option<String>,
    next_cursor: Option<String>,
}

impl ConversationsListResponseConversationsPaginationBuilder {
    pub fn prev_cursor(mut self, value: impl Into<String>) -> Self {
        self.prev_cursor = Some(value.into());
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConversationsListResponseConversationsPagination`].
    pub fn build(self) -> Result<ConversationsListResponseConversationsPagination, BuildError> {
        Ok(ConversationsListResponseConversationsPagination {
            prev_cursor: self.prev_cursor,
            next_cursor: self.next_cursor,
        })
    }
}
