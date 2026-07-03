pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Task {
    /// The name of the task.
    #[serde(default)]
    pub name: String,
    /// The description of the task.
    #[serde(default)]
    pub description: String,
}

impl Task {
    pub fn builder() -> TaskBuilder {
        <TaskBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaskBuilder {
    name: Option<String>,
    description: Option<String>,
}

impl TaskBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Task`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](TaskBuilder::name)
    /// - [`description`](TaskBuilder::description)
    pub fn build(self) -> Result<Task, BuildError> {
        Ok(Task {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}
