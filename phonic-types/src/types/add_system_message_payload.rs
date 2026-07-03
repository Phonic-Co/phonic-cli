pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AddSystemMessagePayload {
    pub r#type: String,
    /// New system message
    #[serde(default)]
    pub system_message: String,
}

impl AddSystemMessagePayload {
    pub fn builder() -> AddSystemMessagePayloadBuilder {
        <AddSystemMessagePayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddSystemMessagePayloadBuilder {
    r#type: Option<String>,
    system_message: Option<String>,
}

impl AddSystemMessagePayloadBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn system_message(mut self, value: impl Into<String>) -> Self {
        self.system_message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddSystemMessagePayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AddSystemMessagePayloadBuilder::r#type)
    /// - [`system_message`](AddSystemMessagePayloadBuilder::system_message)
    pub fn build(self) -> Result<AddSystemMessagePayload, BuildError> {
        Ok(AddSystemMessagePayload {
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            system_message: self.system_message.ok_or_else(|| BuildError::missing_field("system_message"))?,
        })
    }
}
