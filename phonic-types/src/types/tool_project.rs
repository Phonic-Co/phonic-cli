pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolProject {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
}

impl ToolProject {
    pub fn builder() -> ToolProjectBuilder {
        <ToolProjectBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolProjectBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ToolProjectBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolProject`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ToolProjectBuilder::id)
    /// - [`name`](ToolProjectBuilder::name)
    pub fn build(self) -> Result<ToolProject, BuildError> {
        Ok(ToolProject {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
