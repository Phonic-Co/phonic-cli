pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AudioChunkPayload {
    pub r#type: String,
    /// Base64-encoded audio data (Int16Array for PCM, Uint8Array for mulaw). Each chunk may contain at most 40 ms of audio — longer chunks are rejected with an error. Batch ~20 ms frames for headroom.
    #[serde(default)]
    pub audio: String,
    /// ISO 8601 timestamp (required for first chunk only)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub iso_date_time: Option<DateTime<FixedOffset>>,
}

impl AudioChunkPayload {
    pub fn builder() -> AudioChunkPayloadBuilder {
        <AudioChunkPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudioChunkPayloadBuilder {
    r#type: Option<String>,
    audio: Option<String>,
    iso_date_time: Option<DateTime<FixedOffset>>,
}

impl AudioChunkPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn audio(mut self, value: impl Into<String>) -> Self {
        self.audio = Some(value.into());
        self
    }

    pub fn iso_date_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.iso_date_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AudioChunkPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AudioChunkPayloadBuilder::r#type)
    /// - [`audio`](AudioChunkPayloadBuilder::audio)
    pub fn build(self) -> Result<AudioChunkPayload, BuildError> {
        Ok(AudioChunkPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            audio: self.audio.ok_or_else(|| BuildError::missing_field("audio"))?,
            iso_date_time: self.iso_date_time,
        })
    }
}
