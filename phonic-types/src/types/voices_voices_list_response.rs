pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoicesListResponse {
    #[serde(default)]
    pub voices: Vec<Voice>,
}

impl VoicesListResponse {
    pub fn builder() -> VoicesListResponseBuilder {
        <VoicesListResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoicesListResponseBuilder {
    voices: Option<Vec<Voice>>,
}

impl VoicesListResponseBuilder {
    pub fn voices(mut self, value: Vec<Voice>) -> Self {
        self.voices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoicesListResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voices`](VoicesListResponseBuilder::voices)
    pub fn build(self) -> Result<VoicesListResponse, BuildError> {
        Ok(VoicesListResponse {
            voices: self.voices.ok_or_else(|| BuildError::missing_field("voices"))?,
        })
    }
}
