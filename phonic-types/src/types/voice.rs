pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Voice {
    /// The voice ID.
    #[serde(default)]
    pub id: String,
    /// The voice name.
    #[serde(default)]
    pub name: String,
    /// The voice description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Presigned URL to the voice sample audio file. Expires in 7 days.
    #[serde(default)]
    pub audio_url: String,
}

impl Voice {
    pub fn builder() -> VoiceBuilder {
        <VoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoiceBuilder {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    audio_url: Option<String>,
}

impl VoiceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn audio_url(mut self, value: impl Into<String>) -> Self {
        self.audio_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Voice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](VoiceBuilder::id)
    /// - [`name`](VoiceBuilder::name)
    /// - [`audio_url`](VoiceBuilder::audio_url)
    pub fn build(self) -> Result<Voice, BuildError> {
        Ok(Voice {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            audio_url: self.audio_url.ok_or_else(|| BuildError::missing_field("audio_url"))?,
        })
    }
}
