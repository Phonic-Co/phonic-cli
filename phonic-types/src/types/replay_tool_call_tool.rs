pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplayToolCallTool {
    /// The tool ID.
    #[serde(default)]
    pub id: String,
    /// The tool name.
    #[serde(default)]
    pub name: String,
}

impl ReplayToolCallTool {
    pub fn builder() -> ReplayToolCallToolBuilder {
        <ReplayToolCallToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplayToolCallToolBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl ReplayToolCallToolBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReplayToolCallTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReplayToolCallToolBuilder::id)
    /// - [`name`](ReplayToolCallToolBuilder::name)
    pub fn build(self) -> Result<ReplayToolCallTool, BuildError> {
        Ok(ReplayToolCallTool {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
