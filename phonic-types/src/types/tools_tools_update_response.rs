pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolsUpdateResponse {
    /// Whether the tool was updated successfully.
    #[serde(default)]
    pub success: bool,
}

impl ToolsUpdateResponse {
    pub fn builder() -> ToolsUpdateResponseBuilder {
        <ToolsUpdateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolsUpdateResponseBuilder {
    success: Option<bool>,
}

impl ToolsUpdateResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ToolsUpdateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ToolsUpdateResponseBuilder::success)
    pub fn build(self) -> Result<ToolsUpdateResponse, BuildError> {
        Ok(ToolsUpdateResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
