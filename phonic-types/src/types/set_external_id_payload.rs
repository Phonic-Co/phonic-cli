pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetExternalIdPayload {
    pub r#type: String,
    /// External ID to associate with conversation
    #[serde(default)]
    pub external_id: String,
}

impl SetExternalIdPayload {
    pub fn builder() -> SetExternalIdPayloadBuilder {
        <SetExternalIdPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetExternalIdPayloadBuilder {
    r#type: Option<String>,
    external_id: Option<String>,
}

impl SetExternalIdPayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetExternalIdPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](SetExternalIdPayloadBuilder::r#type)
    /// - [`external_id`](SetExternalIdPayloadBuilder::external_id)
    pub fn build(self) -> Result<SetExternalIdPayload, BuildError> {
        Ok(SetExternalIdPayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            external_id: self.external_id.ok_or_else(|| BuildError::missing_field("external_id"))?,
        })
    }
}
