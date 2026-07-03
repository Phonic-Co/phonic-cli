pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProjectRequest {
    /// The name of the project. Can only contain lowercase letters, numbers and hyphens. Must be unique within the workspace.
    #[serde(default)]
    pub name: String,
}

impl CreateProjectRequest {
    pub fn builder() -> CreateProjectRequestBuilder {
        <CreateProjectRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProjectRequestBuilder {
    name: Option<String>,
}

impl CreateProjectRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateProjectRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateProjectRequestBuilder::name)
    pub fn build(self) -> Result<CreateProjectRequest, BuildError> {
        Ok(CreateProjectRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}

