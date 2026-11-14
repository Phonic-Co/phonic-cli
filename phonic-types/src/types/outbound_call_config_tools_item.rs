pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OutboundCallConfigToolsItem {
        String(String),

        ResponsesBuiltInTool(ResponsesBuiltInTool),
}

impl OutboundCallConfigToolsItem {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_responses_built_in_tool(&self) -> bool {
        matches!(self, Self::ResponsesBuiltInTool(_))
    }


    pub fn as_string(&self) -> Option<&str> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
                    Self::String(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_responses_built_in_tool(&self) -> Option<&ResponsesBuiltInTool> {
        match self {
                    Self::ResponsesBuiltInTool(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_responses_built_in_tool(self) -> Option<ResponsesBuiltInTool> {
        match self {
                    Self::ResponsesBuiltInTool(value) => Some(value),
                    _ => None,
                }
    }
}
