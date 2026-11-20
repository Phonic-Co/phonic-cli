pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A pointer to the tool that was called by the assistant (see ResponsesToolCall).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResponsesToolReference {
    /// Optional tool ID, accepted for compatibility with tool calls copied from conversation items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Name of the tool that was called.
    #[serde(default)]
    pub name: String,
}

impl ResponsesToolReference {
    pub fn builder() -> ResponsesToolReferenceBuilder {
        <ResponsesToolReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponsesToolReferenceBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ResponsesToolReferenceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponsesToolReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ResponsesToolReferenceBuilder::name)
    pub fn build(self) -> Result<ResponsesToolReference, BuildError> {
        Ok(ResponsesToolReference {
            id: self.id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
