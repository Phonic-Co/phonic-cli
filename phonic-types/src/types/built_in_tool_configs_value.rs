pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum BuiltInToolConfigsValue {
        BuiltInToolConfig(BuiltInToolConfig),

        ChooseNotToRespondToolConfig(ChooseNotToRespondToolConfig),
}

impl BuiltInToolConfigsValue {
    pub fn is_built_in_tool_config(&self) -> bool {
        matches!(self, Self::BuiltInToolConfig(_))
    }

    pub fn is_choose_not_to_respond_tool_config(&self) -> bool {
        matches!(self, Self::ChooseNotToRespondToolConfig(_))
    }


    pub fn as_built_in_tool_config(&self) -> Option<&BuiltInToolConfig> {
        match self {
                    Self::BuiltInToolConfig(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_built_in_tool_config(self) -> Option<BuiltInToolConfig> {
        match self {
                    Self::BuiltInToolConfig(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_choose_not_to_respond_tool_config(&self) -> Option<&ChooseNotToRespondToolConfig> {
        match self {
                    Self::ChooseNotToRespondToolConfig(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_choose_not_to_respond_tool_config(self) -> Option<ChooseNotToRespondToolConfig> {
        match self {
                    Self::ChooseNotToRespondToolConfig(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for BuiltInToolConfigsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BuiltInToolConfig(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ChooseNotToRespondToolConfig(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
