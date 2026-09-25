pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResponsesTool {
        ResponsesToolName(ResponsesToolName),

        ResponsesBuiltInTool(ResponsesBuiltInTool),
}

impl ResponsesTool {
    pub fn is_responses_tool_name(&self) -> bool {
        matches!(self, Self::ResponsesToolName(_))
    }

    pub fn is_responses_built_in_tool(&self) -> bool {
        matches!(self, Self::ResponsesBuiltInTool(_))
    }


    pub fn as_responses_tool_name(&self) -> Option<&ResponsesToolName> {
        match self {
                    Self::ResponsesToolName(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_responses_tool_name(self) -> Option<ResponsesToolName> {
        match self {
                    Self::ResponsesToolName(value) => Some(value),
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

impl fmt::Display for ResponsesTool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ResponsesToolName(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ResponsesBuiltInTool(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
