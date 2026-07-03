pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DtmfPayload {
    pub r#type: String,
    /// DTMF digits to play
    #[serde(default)]
    pub digits: String,
    /// Optional latency-breakdown metrics for the message, expressed as named millisecond durations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timings: Option<HashMap<String, f64>>,
}

impl DtmfPayload {
    pub fn builder() -> DtmfPayloadBuilder {
        <DtmfPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DtmfPayloadBuilder {
    r#type: Option<String>,
    digits: Option<String>,
    timings: Option<HashMap<String, f64>>,
}

impl DtmfPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn digits(mut self, value: impl Into<String>) -> Self {
        self.digits = Some(value.into());
        self
    }

    pub fn timings(mut self, value: HashMap<String, f64>) -> Self {
        self.timings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DtmfPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](DtmfPayloadBuilder::r#type)
    /// - [`digits`](DtmfPayloadBuilder::digits)
    pub fn build(self) -> Result<DtmfPayload, BuildError> {
        Ok(DtmfPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            digits: self.digits.ok_or_else(|| BuildError::missing_field("digits"))?,
            timings: self.timings,
        })
    }
}
