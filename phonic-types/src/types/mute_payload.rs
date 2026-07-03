pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MutePayload {
    pub r#type: String,
}

impl MutePayload {
    pub fn builder() -> MutePayloadBuilder {
        <MutePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutePayloadBuilder {
    r#type: Option<String>,
}

impl MutePayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MutePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](MutePayloadBuilder::r#type)
    pub fn build(self) -> Result<MutePayload, BuildError> {
        Ok(MutePayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
