pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conversation {
    /// The conversation ID.
    #[serde(default)]
    pub id: String,
    /// The agent associated with the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<ConversationAgent>,
    /// The organization/workspace name.
    #[serde(default)]
    pub workspace: String,
    /// The project associated with the conversation.
    #[serde(default)]
    pub project: ConversationProject,
    /// External ID for conversation tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The origin of the conversation.
    pub origin: ConversationOrigin,
    /// The STS model used.
    #[serde(default)]
    pub model: String,
    /// Will be `true` if welcome message was automatically generated.
    #[serde(default)]
    pub generate_welcome_message: bool,
    /// When `false`, the welcome message will not be interruptible by the user.
    #[serde(default)]
    pub is_welcome_message_interruptible: bool,
    /// Welcome message played at start. Will be `null` when `generate_welcome_message` is `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_message: Option<String>,
    /// Template variables used in the conversation.
    #[serde(default)]
    pub template_variables: HashMap<String, String>,
    /// System prompt used in the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    /// Audio input format.
    #[serde(default)]
    pub input_format: String,
    /// Audio output format.
    #[serde(default)]
    pub output_format: String,
    /// Background noise level used in the conversation.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub background_noise_level: f64,
    /// The background noise type used in the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_noise: Option<ConversationBackgroundNoise>,
    /// Live transcript of the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_transcript: Option<String>,
    /// Post-call processed transcript.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_call_transcript: Option<String>,
    /// Duration of the conversation in milliseconds.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub duration_ms: f64,
    /// Presigned URL to the conversation audio file. Expires in 1 day.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_url: Option<String>,
    /// When the conversation started.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub started_at: Option<DateTime<FixedOffset>>,
    /// When the conversation ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub ended_at: Option<DateTime<FixedOffset>>,
    /// Who or what ended the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_by: Option<ConversationEndedBy>,
    /// These words, or short phrases, are more accurately recognized by the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boosted_keywords: Option<Vec<String>>,
    /// Array of `{ word, pronunciation }` entries. Words must be unique.
    #[serde(default)]
    pub pronunciation_dictionary: Vec<ConversationPronunciationDictionaryItem>,
    /// Minimum number of words required to interrupt the assistant.
    #[serde(default)]
    pub min_words_to_interrupt: i64,
    /// ISO 639-1 language code that sets the agent's default language to recognize and speak. Welcome message and no input poke text should be in this language.
    pub default_language: LanguageCode,
    /// Array of additional ISO 639-1 language codes that the agent should be able to recognize and speak. Should not include `default_language`. When `multilingual_mode` is `"auto"`, a maximum of 2 additional languages is allowed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_languages: Option<Vec<LanguageCode>>,
    /// If `"auto"`, each user audio is automatically identified for the language to respond in. If `"request"`, user must request to change language (recommended). If `"initial"` the first turn user audio determines the language for the rest of the conversation.
    pub multilingual_mode: ConversationMultilingualMode,
    /// Push to talk mode. User must send mute/unmute messages to turn on/off listening to audio. Defaults to false.
    #[serde(default)]
    pub push_to_talk: bool,
    /// Array of ISO 639-1 language codes recognized by the model. This field is deprecated. Use `default_language` and `additional_languages` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<String>>,
    /// Whether the no-input poke text was generated by AI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_no_input_poke_text: Option<bool>,
    /// Number of seconds of silence before a poke message is sent. `null` means the poke message is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_input_poke_sec: Option<i64>,
    /// The message to send after the specified silence. Relevant only if `no_input_poke_sec` is not `null`. Ignored when generate_no_input_poke_text is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_input_poke_text: Option<String>,
    /// Seconds of silence before the conversation is ended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_input_end_conversation_sec: Option<i64>,
    /// The WebSocket idle timeout in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub websocket_timeout_sec: Option<f64>,
    /// Voice activity detection prebuffer duration in milliseconds. `null` when not applicable or unknown (e.g. push-to-talk, or legacy stored conversations).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad_prebuffer_duration_ms: Option<i64>,
    /// Minimum speech duration for voice activity detection in milliseconds. `null` when not applicable or unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad_min_speech_duration_ms: Option<i64>,
    /// Minimum silence duration for voice activity detection in milliseconds. `null` when not applicable or unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad_min_silence_duration_ms: Option<i64>,
    /// Voice activity detection threshold. `null` when not applicable or unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad_threshold: Option<f64>,
    /// Results from conversation evaluations and extractions.
    #[serde(default)]
    pub task_results: HashMap<String, serde_json::Value>,
    /// Array of conversation items (turns).
    #[serde(default)]
    pub items: Vec<ConversationItem>,
    /// Phone call metadata. `null` for non-phone call conversations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_info: Option<ConversationCallInfo>,
    /// Analysis of the conversation including latencies and interruptions.
    #[serde(default)]
    pub analysis: ConversationAnalysis,
    /// Whether PII and PHI have been redacted from the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_redacted: Option<bool>,
    /// The redacted transcript of the conversation. `null` when the conversation is not redacted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redacted_transcript: Option<String>,
    /// Arbitrary metadata associated with the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Controls how long transcripts and audio recordings are retained before deletion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_retention_policy: Option<DataRetentionPolicy>,
    /// Information about when transcripts and audio recordings are or were scheduled to be deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_info: Option<ConversationDeletionInfo>,
    /// Whether the assistant produced backchannel responses during the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_assistant_backchannel: Option<bool>,
    /// How aggressively the assistant produced backchannel responses during the conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assistant_backchannel_aggressiveness: Option<f64>,
}

impl Conversation {
    pub fn builder() -> ConversationBuilder {
        <ConversationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationBuilder {
    id: Option<String>,
    agent: Option<ConversationAgent>,
    workspace: Option<String>,
    project: Option<ConversationProject>,
    external_id: Option<String>,
    origin: Option<ConversationOrigin>,
    model: Option<String>,
    generate_welcome_message: Option<bool>,
    is_welcome_message_interruptible: Option<bool>,
    welcome_message: Option<String>,
    template_variables: Option<HashMap<String, String>>,
    system_prompt: Option<String>,
    input_format: Option<String>,
    output_format: Option<String>,
    background_noise_level: Option<f64>,
    background_noise: Option<ConversationBackgroundNoise>,
    live_transcript: Option<String>,
    post_call_transcript: Option<String>,
    duration_ms: Option<f64>,
    audio_url: Option<String>,
    started_at: Option<DateTime<FixedOffset>>,
    ended_at: Option<DateTime<FixedOffset>>,
    ended_by: Option<ConversationEndedBy>,
    boosted_keywords: Option<Vec<String>>,
    pronunciation_dictionary: Option<Vec<ConversationPronunciationDictionaryItem>>,
    min_words_to_interrupt: Option<i64>,
    default_language: Option<LanguageCode>,
    additional_languages: Option<Vec<LanguageCode>>,
    multilingual_mode: Option<ConversationMultilingualMode>,
    push_to_talk: Option<bool>,
    languages: Option<Vec<String>>,
    generate_no_input_poke_text: Option<bool>,
    no_input_poke_sec: Option<i64>,
    no_input_poke_text: Option<String>,
    no_input_end_conversation_sec: Option<i64>,
    websocket_timeout_sec: Option<f64>,
    vad_prebuffer_duration_ms: Option<i64>,
    vad_min_speech_duration_ms: Option<i64>,
    vad_min_silence_duration_ms: Option<i64>,
    vad_threshold: Option<f64>,
    task_results: Option<HashMap<String, serde_json::Value>>,
    items: Option<Vec<ConversationItem>>,
    call_info: Option<ConversationCallInfo>,
    analysis: Option<ConversationAnalysis>,
    is_redacted: Option<bool>,
    redacted_transcript: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    data_retention_policy: Option<DataRetentionPolicy>,
    deletion_info: Option<ConversationDeletionInfo>,
    enable_assistant_backchannel: Option<bool>,
    assistant_backchannel_aggressiveness: Option<f64>,
}

impl ConversationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn agent(mut self, value: ConversationAgent) -> Self {
        self.agent = Some(value);
        self
    }

    pub fn workspace(mut self, value: impl Into<String>) -> Self {
        self.workspace = Some(value.into());
        self
    }

    pub fn project(mut self, value: ConversationProject) -> Self {
        self.project = Some(value);
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn origin(mut self, value: ConversationOrigin) -> Self {
        self.origin = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn generate_welcome_message(mut self, value: bool) -> Self {
        self.generate_welcome_message = Some(value);
        self
    }

    pub fn is_welcome_message_interruptible(mut self, value: bool) -> Self {
        self.is_welcome_message_interruptible = Some(value);
        self
    }

    pub fn welcome_message(mut self, value: impl Into<String>) -> Self {
        self.welcome_message = Some(value.into());
        self
    }

    pub fn template_variables(mut self, value: HashMap<String, String>) -> Self {
        self.template_variables = Some(value);
        self
    }

    pub fn system_prompt(mut self, value: impl Into<String>) -> Self {
        self.system_prompt = Some(value.into());
        self
    }

    pub fn input_format(mut self, value: impl Into<String>) -> Self {
        self.input_format = Some(value.into());
        self
    }

    pub fn output_format(mut self, value: impl Into<String>) -> Self {
        self.output_format = Some(value.into());
        self
    }

    pub fn background_noise_level(mut self, value: f64) -> Self {
        self.background_noise_level = Some(value);
        self
    }

    pub fn background_noise(mut self, value: ConversationBackgroundNoise) -> Self {
        self.background_noise = Some(value);
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

    pub fn duration_ms(mut self, value: f64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn audio_url(mut self, value: impl Into<String>) -> Self {
        self.audio_url = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn ended_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.ended_at = Some(value);
        self
    }

    pub fn ended_by(mut self, value: ConversationEndedBy) -> Self {
        self.ended_by = Some(value);
        self
    }

    pub fn boosted_keywords(mut self, value: Vec<String>) -> Self {
        self.boosted_keywords = Some(value);
        self
    }

    pub fn pronunciation_dictionary(mut self, value: Vec<ConversationPronunciationDictionaryItem>) -> Self {
        self.pronunciation_dictionary = Some(value);
        self
    }

    pub fn min_words_to_interrupt(mut self, value: i64) -> Self {
        self.min_words_to_interrupt = Some(value);
        self
    }

    pub fn default_language(mut self, value: LanguageCode) -> Self {
        self.default_language = Some(value);
        self
    }

    pub fn additional_languages(mut self, value: Vec<LanguageCode>) -> Self {
        self.additional_languages = Some(value);
        self
    }

    pub fn multilingual_mode(mut self, value: ConversationMultilingualMode) -> Self {
        self.multilingual_mode = Some(value);
        self
    }

    pub fn push_to_talk(mut self, value: bool) -> Self {
        self.push_to_talk = Some(value);
        self
    }

    pub fn languages(mut self, value: Vec<String>) -> Self {
        self.languages = Some(value);
        self
    }

    pub fn generate_no_input_poke_text(mut self, value: bool) -> Self {
        self.generate_no_input_poke_text = Some(value);
        self
    }

    pub fn no_input_poke_sec(mut self, value: i64) -> Self {
        self.no_input_poke_sec = Some(value);
        self
    }

    pub fn no_input_poke_text(mut self, value: impl Into<String>) -> Self {
        self.no_input_poke_text = Some(value.into());
        self
    }

    pub fn no_input_end_conversation_sec(mut self, value: i64) -> Self {
        self.no_input_end_conversation_sec = Some(value);
        self
    }

    pub fn websocket_timeout_sec(mut self, value: f64) -> Self {
        self.websocket_timeout_sec = Some(value);
        self
    }

    pub fn vad_prebuffer_duration_ms(mut self, value: i64) -> Self {
        self.vad_prebuffer_duration_ms = Some(value);
        self
    }

    pub fn vad_min_speech_duration_ms(mut self, value: i64) -> Self {
        self.vad_min_speech_duration_ms = Some(value);
        self
    }

    pub fn vad_min_silence_duration_ms(mut self, value: i64) -> Self {
        self.vad_min_silence_duration_ms = Some(value);
        self
    }

    pub fn vad_threshold(mut self, value: f64) -> Self {
        self.vad_threshold = Some(value);
        self
    }

    pub fn task_results(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.task_results = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<ConversationItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn call_info(mut self, value: ConversationCallInfo) -> Self {
        self.call_info = Some(value);
        self
    }

    pub fn analysis(mut self, value: ConversationAnalysis) -> Self {
        self.analysis = Some(value);
        self
    }

    pub fn is_redacted(mut self, value: bool) -> Self {
        self.is_redacted = Some(value);
        self
    }

    pub fn redacted_transcript(mut self, value: impl Into<String>) -> Self {
        self.redacted_transcript = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn data_retention_policy(mut self, value: DataRetentionPolicy) -> Self {
        self.data_retention_policy = Some(value);
        self
    }

    pub fn deletion_info(mut self, value: ConversationDeletionInfo) -> Self {
        self.deletion_info = Some(value);
        self
    }

    pub fn enable_assistant_backchannel(mut self, value: bool) -> Self {
        self.enable_assistant_backchannel = Some(value);
        self
    }

    pub fn assistant_backchannel_aggressiveness(mut self, value: f64) -> Self {
        self.assistant_backchannel_aggressiveness = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Conversation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ConversationBuilder::id)
    /// - [`workspace`](ConversationBuilder::workspace)
    /// - [`project`](ConversationBuilder::project)
    /// - [`origin`](ConversationBuilder::origin)
    /// - [`model`](ConversationBuilder::model)
    /// - [`generate_welcome_message`](ConversationBuilder::generate_welcome_message)
    /// - [`is_welcome_message_interruptible`](ConversationBuilder::is_welcome_message_interruptible)
    /// - [`template_variables`](ConversationBuilder::template_variables)
    /// - [`input_format`](ConversationBuilder::input_format)
    /// - [`output_format`](ConversationBuilder::output_format)
    /// - [`background_noise_level`](ConversationBuilder::background_noise_level)
    /// - [`duration_ms`](ConversationBuilder::duration_ms)
    /// - [`pronunciation_dictionary`](ConversationBuilder::pronunciation_dictionary)
    /// - [`min_words_to_interrupt`](ConversationBuilder::min_words_to_interrupt)
    /// - [`default_language`](ConversationBuilder::default_language)
    /// - [`multilingual_mode`](ConversationBuilder::multilingual_mode)
    /// - [`push_to_talk`](ConversationBuilder::push_to_talk)
    /// - [`task_results`](ConversationBuilder::task_results)
    /// - [`items`](ConversationBuilder::items)
    /// - [`analysis`](ConversationBuilder::analysis)
    pub fn build(self) -> Result<Conversation, BuildError> {
        Ok(Conversation {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            agent: self.agent,
            workspace: self.workspace.ok_or_else(|| BuildError::missing_field("workspace"))?,
            project: self.project.ok_or_else(|| BuildError::missing_field("project"))?,
            external_id: self.external_id,
            origin: self.origin.ok_or_else(|| BuildError::missing_field("origin"))?,
            model: self.model.ok_or_else(|| BuildError::missing_field("model"))?,
            generate_welcome_message: self.generate_welcome_message.ok_or_else(|| BuildError::missing_field("generate_welcome_message"))?,
            is_welcome_message_interruptible: self.is_welcome_message_interruptible.ok_or_else(|| BuildError::missing_field("is_welcome_message_interruptible"))?,
            welcome_message: self.welcome_message,
            template_variables: self.template_variables.ok_or_else(|| BuildError::missing_field("template_variables"))?,
            system_prompt: self.system_prompt,
            input_format: self.input_format.ok_or_else(|| BuildError::missing_field("input_format"))?,
            output_format: self.output_format.ok_or_else(|| BuildError::missing_field("output_format"))?,
            background_noise_level: self.background_noise_level.ok_or_else(|| BuildError::missing_field("background_noise_level"))?,
            background_noise: self.background_noise,
            live_transcript: self.live_transcript,
            post_call_transcript: self.post_call_transcript,
            duration_ms: self.duration_ms.ok_or_else(|| BuildError::missing_field("duration_ms"))?,
            audio_url: self.audio_url,
            started_at: self.started_at,
            ended_at: self.ended_at,
            ended_by: self.ended_by,
            boosted_keywords: self.boosted_keywords,
            pronunciation_dictionary: self.pronunciation_dictionary.ok_or_else(|| BuildError::missing_field("pronunciation_dictionary"))?,
            min_words_to_interrupt: self.min_words_to_interrupt.ok_or_else(|| BuildError::missing_field("min_words_to_interrupt"))?,
            default_language: self.default_language.ok_or_else(|| BuildError::missing_field("default_language"))?,
            additional_languages: self.additional_languages,
            multilingual_mode: self.multilingual_mode.ok_or_else(|| BuildError::missing_field("multilingual_mode"))?,
            push_to_talk: self.push_to_talk.ok_or_else(|| BuildError::missing_field("push_to_talk"))?,
            languages: self.languages,
            generate_no_input_poke_text: self.generate_no_input_poke_text,
            no_input_poke_sec: self.no_input_poke_sec,
            no_input_poke_text: self.no_input_poke_text,
            no_input_end_conversation_sec: self.no_input_end_conversation_sec,
            websocket_timeout_sec: self.websocket_timeout_sec,
            vad_prebuffer_duration_ms: self.vad_prebuffer_duration_ms,
            vad_min_speech_duration_ms: self.vad_min_speech_duration_ms,
            vad_min_silence_duration_ms: self.vad_min_silence_duration_ms,
            vad_threshold: self.vad_threshold,
            task_results: self.task_results.ok_or_else(|| BuildError::missing_field("task_results"))?,
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            call_info: self.call_info,
            analysis: self.analysis.ok_or_else(|| BuildError::missing_field("analysis"))?,
            is_redacted: self.is_redacted,
            redacted_transcript: self.redacted_transcript,
            metadata: self.metadata,
            data_retention_policy: self.data_retention_policy,
            deletion_info: self.deletion_info,
            enable_assistant_backchannel: self.enable_assistant_backchannel,
            assistant_backchannel_aggressiveness: self.assistant_backchannel_aggressiveness,
        })
    }
}
