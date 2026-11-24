pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TtsResponse {
    /// The generated speech, base64-encoded in the requested `output_format`.
    #[serde(default)]
    pub audio: String,
}

impl TtsResponse {
    pub fn builder() -> TtsResponseBuilder {
        <TtsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TtsResponseBuilder {
    audio: Option<String>,
}

impl TtsResponseBuilder {
    pub fn audio(mut self, value: impl Into<String>) -> Self {
        self.audio = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TtsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audio`](TtsResponseBuilder::audio)
    pub fn build(self) -> Result<TtsResponse, BuildError> {
        Ok(TtsResponse {
            audio: self.audio.ok_or_else(|| BuildError::missing_field("audio"))?,
        })
    }
}
