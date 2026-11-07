pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The tool the assistant wants to call.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeneratedToolReference {
    /// Name of the tool to call. Always one of the `tool_definitions` from the request.
    #[serde(default)]
    pub name: String,
}

impl GeneratedToolReference {
    pub fn builder() -> GeneratedToolReferenceBuilder {
        <GeneratedToolReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneratedToolReferenceBuilder {
    name: Option<String>,
}

impl GeneratedToolReferenceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GeneratedToolReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](GeneratedToolReferenceBuilder::name)
    pub fn build(self) -> Result<GeneratedToolReference, BuildError> {
        Ok(GeneratedToolReference {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
