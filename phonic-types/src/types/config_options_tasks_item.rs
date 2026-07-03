pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigOptionsTasksItem {
    /// Name of the task.
    #[serde(default)]
    pub name: String,
    /// Description of the task.
    #[serde(default)]
    pub description: String,
}

impl ConfigOptionsTasksItem {
    pub fn builder() -> ConfigOptionsTasksItemBuilder {
        <ConfigOptionsTasksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigOptionsTasksItemBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl ConfigOptionsTasksItemBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConfigOptionsTasksItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](ConfigOptionsTasksItemBuilder::name)
    /// - [`description`](ConfigOptionsTasksItemBuilder::description)
    pub fn build(self) -> Result<ConfigOptionsTasksItem, BuildError> {
        Ok(ConfigOptionsTasksItem {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
