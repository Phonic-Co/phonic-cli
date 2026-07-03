pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioChunkResponsePayload {
    pub r#type: String,
    /// Base64-encoded AI audio data
    #[serde(default)]
    pub audio: String,
    /// Text corresponding to audio chunk
    #[serde(default)]
    pub text: String,
    /// Optional latency-breakdown metrics for the audio chunk, expressed as named millisecond durations. Only present on the first audio chunk of a turn.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timings: Option<HashMap<String, f64>>,
}

impl AudioChunkResponsePayload {
    pub fn builder() -> AudioChunkResponsePayloadBuilder {
        <AudioChunkResponsePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioChunkResponsePayloadBuilder {
    r#type: Option<String>,
    audio: Option<String>,
    text: Option<String>,
    timings: Option<HashMap<String, f64>>,
}

impl AudioChunkResponsePayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn audio(mut self, value: impl Into<String>) -> Self {
        self.audio = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn timings(mut self, value: HashMap<String, f64>) -> Self {
        self.timings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioChunkResponsePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AudioChunkResponsePayloadBuilder::r#type)
    /// - [`audio`](AudioChunkResponsePayloadBuilder::audio)
    /// - [`text`](AudioChunkResponsePayloadBuilder::text)
    pub fn build(self) -> Result<AudioChunkResponsePayload, BuildError> {
        Ok(AudioChunkResponsePayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            audio: self.audio.ok_or_else(|| BuildError::missing_field("audio"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            timings: self.timings,
        })
    }
}
