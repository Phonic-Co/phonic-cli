pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConversationsListQueryRequest {
    /// The name of the project to list conversations for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    /// Filter by external ID to get a specific conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Minimum duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_min: Option<i64>,
    /// Maximum duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_max: Option<i64>,
    /// Minimum start date/time. Valid examples: `2025-04-17`, `2025-04-17T02:48:52.708Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at_min: Option<String>,
    /// Maximum start date/time. Valid examples: `2025-04-17`, `2025-04-17T02:48:52.708Z`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at_max: Option<String>,
    /// Cursor for backward pagination. Use a conversation ID from `pagination.prev_cursor` to fetch the previous page of conversations. Cannot be used with `after`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Cursor for forward pagination. Use a conversation ID from `pagination.next_cursor` to fetch the next page of conversations. Cannot be used with `before`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Maximum number of conversations to return per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Format of the presigned `audio_url` in each conversation in the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_container: Option<ConversationsListRequestAudioContainer>,
}

impl ConversationsListQueryRequest {
    pub fn builder() -> ConversationsListQueryRequestBuilder {
        <ConversationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationsListQueryRequestBuilder {
    project: Option<String>,
    external_id: Option<String>,
    duration_min: Option<i64>,
    duration_max: Option<i64>,
    started_at_min: Option<String>,
    started_at_max: Option<String>,
    before: Option<String>,
    after: Option<String>,
    limit: Option<i64>,
    audio_container: Option<ConversationsListRequestAudioContainer>,
}

impl ConversationsListQueryRequestBuilder {
    pub fn project(mut self, value: impl Into<String>) -> Self {
        self.project = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn duration_min(mut self, value: i64) -> Self {
        self.duration_min = Some(value);
        self
    }

    pub fn duration_max(mut self, value: i64) -> Self {
        self.duration_max = Some(value);
        self
    }

    pub fn started_at_min(mut self, value: impl Into<String>) -> Self {
        self.started_at_min = Some(value.into());
        self
    }

    pub fn started_at_max(mut self, value: impl Into<String>) -> Self {
        self.started_at_max = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn audio_container(mut self, value: ConversationsListRequestAudioContainer) -> Self {
        self.audio_container = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationsListQueryRequest`].
    pub fn build(self) -> Result<ConversationsListQueryRequest, BuildError> {
        Ok(ConversationsListQueryRequest {
            project: self.project,
            external_id: self.external_id,
            duration_min: self.duration_min,
            duration_max: self.duration_max,
            started_at_min: self.started_at_min,
            started_at_max: self.started_at_max,
            before: self.before,
            after: self.after,
            limit: self.limit,
            audio_container: self.audio_container,
        })
    }
}

