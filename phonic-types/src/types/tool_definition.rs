pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolDefinition {
        ToolName(ToolName),

        InlineWebSocketTool(InlineWebSocketTool),
}

impl ToolDefinition {
    pub fn is_tool_name(&self) -> bool {
        matches!(self, Self::ToolName(_))
    }

    pub fn is_inline_web_socket_tool(&self) -> bool {
        matches!(self, Self::InlineWebSocketTool(_))
    }


    pub fn as_tool_name(&self) -> Option<&ToolName> {
        match self {
                    Self::ToolName(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tool_name(self) -> Option<ToolName> {
        match self {
                    Self::ToolName(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_inline_web_socket_tool(&self) -> Option<&InlineWebSocketTool> {
        match self {
                    Self::InlineWebSocketTool(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_inline_web_socket_tool(self) -> Option<InlineWebSocketTool> {
        match self {
                    Self::InlineWebSocketTool(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ToolDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolName(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::InlineWebSocketTool(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
