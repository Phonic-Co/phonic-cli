pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolsCreateResponse {
    /// The ID of the created tool.
    #[serde(default)]
    pub id: String,
    /// The name of the created tool.
    #[serde(default)]
    pub name: String,
}

impl ToolsCreateResponse {
    pub fn builder() -> ToolsCreateResponseBuilder {
        <ToolsCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsCreateResponseBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ToolsCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolsCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ToolsCreateResponseBuilder::id)
    /// - [`name`](ToolsCreateResponseBuilder::name)
    pub fn build(self) -> Result<ToolsCreateResponse, BuildError> {
        Ok(ToolsCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
