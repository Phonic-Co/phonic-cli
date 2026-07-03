pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConversationItem {
    /// The conversation item ID.
    #[serde(default)]
    pub id: String,
    /// Index of the item in the conversation.
    #[serde(default)]
    pub item_idx: i64,
    /// Who spoke in this turn.
    pub role: ConversationItemRole,
    /// Live transcript of this turn. `null` when the turn has been redacted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_transcript: Option<String>,
    /// Post-call processed transcript.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_call_transcript: Option<String>,
    /// The redacted transcript of this turn. `null` when the turn is not redacted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_transcript: Option<String>,
    /// Duration of this turn in milliseconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub duration_ms: f64,
    /// When this turn started.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub started_at: DateTime<FixedOffset>,
    /// Voice ID used (assistant only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// Audio speed used (assistant only).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub audio_speed: Option<f64>,
    /// System prompt used for this assistant turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    /// Tool calls made by the assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ConversationItemToolCallsItem>>,
}

impl ConversationItem {
    pub fn builder() -> ConversationItemBuilder {
        <ConversationItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationItemBuilder {
    id: Option<String>,
    item_idx: Option<i64>,
    role: Option<ConversationItemRole>,
    live_transcript: Option<String>,
    post_call_transcript: Option<String>,
    redacted_transcript: Option<String>,
    duration_ms: Option<f64>,
    started_at: Option<DateTime<FixedOffset>>,
    voice_id: Option<String>,
    audio_speed: Option<f64>,
    system_prompt: Option<String>,
    tool_calls: Option<Vec<ConversationItemToolCallsItem>>,
}

impl ConversationItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn item_idx(mut self, value: i64) -> Self {
        self.item_idx = Some(value);
        self
    }

    pub fn role(mut self, value: ConversationItemRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn live_transcript(mut self, value: impl Into<String>) -> Self {
        self.live_transcript = Some(value.into());
        self
    }

    pub fn post_call_transcript(mut self, value: impl Into<String>) -> Self {
        self.post_call_transcript = Some(value.into());
        self
    }

    pub fn redacted_transcript(mut self, value: impl Into<String>) -> Self {
        self.redacted_transcript = Some(value.into());
        self
    }

    pub fn duration_ms(mut self, value: f64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn audio_speed(mut self, value: f64) -> Self {
        self.audio_speed = Some(value);
        self
    }

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    pub fn tool_calls(mut self, value: Vec<ConversationItemToolCallsItem>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConversationItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationItemBuilder::id)
    /// - [`item_idx`](ConversationItemBuilder::item_idx)
    /// - [`role`](ConversationItemBuilder::role)
    /// - [`duration_ms`](ConversationItemBuilder::duration_ms)
    /// - [`started_at`](ConversationItemBuilder::started_at)
    pub fn build(self) -> Result<ConversationItem, BuildError> {
        Ok(ConversationItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            item_idx: self.item_idx.ok_or_else(|| BuildError::missing_field("item_idx"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            live_transcript: self.live_transcript,
            post_call_transcript: self.post_call_transcript,
            redacted_transcript: self.redacted_transcript,
            duration_ms: self.duration_ms.ok_or_else(|| BuildError::missing_field("duration_ms"))?,
            started_at: self.started_at.ok_or_else(|| BuildError::missing_field("started_at"))?,
            voice_id: self.voice_id,
            audio_speed: self.audio_speed,
            system_prompt: self.system_prompt,
            tool_calls: self.tool_calls,
        })
    }
}
