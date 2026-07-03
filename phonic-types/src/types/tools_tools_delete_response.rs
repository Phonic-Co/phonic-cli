pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolsDeleteResponse {
    /// Whether the tool was deleted successfully.
    #[serde(default)]
    pub success: bool,
}

impl ToolsDeleteResponse {
    pub fn builder() -> ToolsDeleteResponseBuilder {
        <ToolsDeleteResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsDeleteResponseBuilder {
    success: Option<bool>,
}

impl ToolsDeleteResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolsDeleteResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ToolsDeleteResponseBuilder::success)
    pub fn build(self) -> Result<ToolsDeleteResponse, BuildError> {
        Ok(ToolsDeleteResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
