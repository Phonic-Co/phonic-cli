pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StreamTtsRequest {
    /// The text to convert to speech.
    #[serde(default)]
    pub text: String,
    /// The TTS model to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The speech speed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub speed: Option<f64>,
    /// The voice ID to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_id: Option<String>,
    /// The audio format to stream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<StreamTtsRequestOutputFormat>,
}

impl StreamTtsRequest {
    pub fn builder() -> StreamTtsRequestBuilder {
        <StreamTtsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamTtsRequestBuilder {
    text: Option<String>,
    model: Option<String>,
    speed: Option<f64>,
    voice_id: Option<String>,
    output_format: Option<StreamTtsRequestOutputFormat>,
}

impl StreamTtsRequestBuilder {
    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn speed(mut self, value: f64) -> Self {
        self.speed = Some(value);
        self
    }

    pub fn voice_id(mut self, value: impl Into<String>) -> Self {
        self.voice_id = Some(value.into());
        self
    }

    pub fn output_format(mut self, value: StreamTtsRequestOutputFormat) -> Self {
        self.output_format = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamTtsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](StreamTtsRequestBuilder::text)
    pub fn build(self) -> Result<StreamTtsRequest, BuildError> {
        Ok(StreamTtsRequest {
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            model: self.model,
            speed: self.speed,
            voice_id: self.voice_id,
            output_format: self.output_format,
        })
    }
}

