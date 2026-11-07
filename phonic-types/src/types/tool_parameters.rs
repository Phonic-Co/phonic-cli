pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ToolParameters {
        ToolParameterList(Vec<ToolParameter>),

        ToolParametersJsonSchema(ToolParametersJsonSchema),
}

impl ToolParameters {
    pub fn is_tool_parameter_list(&self) -> bool {
        matches!(self, Self::ToolParameterList(_))
    }

    pub fn is_tool_parameters_json_schema(&self) -> bool {
        matches!(self, Self::ToolParametersJsonSchema(_))
    }


    pub fn as_tool_parameter_list(&self) -> Option<&Vec<ToolParameter>> {
        match self {
                    Self::ToolParameterList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tool_parameter_list(self) -> Option<Vec<ToolParameter>> {
        match self {
                    Self::ToolParameterList(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_tool_parameters_json_schema(&self) -> Option<&ToolParametersJsonSchema> {
        match self {
                    Self::ToolParametersJsonSchema(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_tool_parameters_json_schema(self) -> Option<ToolParametersJsonSchema> {
        match self {
                    Self::ToolParametersJsonSchema(value) => Some(value),
                    _ => None,
                }
    }
}
