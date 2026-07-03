pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoicesGetResponse {
    #[serde(default)]
    pub voice: Voice,
}

impl VoicesGetResponse {
    pub fn builder() -> VoicesGetResponseBuilder {
        <VoicesGetResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoicesGetResponseBuilder {
    voice: Option<Voice>,
}

impl VoicesGetResponseBuilder {
    pub fn voice(mut self, value: Voice) -> Self {
        self.voice = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoicesGetResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`voice`](VoicesGetResponseBuilder::voice)
    pub fn build(self) -> Result<VoicesGetResponse, BuildError> {
        Ok(VoicesGetResponse {
            voice: self.voice.ok_or_else(|| BuildError::missing_field("voice"))?,
        })
    }
}
