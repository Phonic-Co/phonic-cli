pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Only applicable for `custom_webhook` tools. Specifies where the parameter should be sent in the webhook request.
/// - For GET webhooks: defaults to `"query_string"` and `"request_body"` is not allowed.
/// - For POST webhooks: required, can be `"request_body"`, `"query_string"`, or `"url_path"`.
/// - `"url_path"` fills a matching `{name}` placeholder in the endpoint URL's path or query (GET and POST). The parameter must be required, and every placeholder in `endpoint_url` must have a matching `url_path` parameter.
/// - Not allowed for `custom_websocket`, `built_in_transfer_to_phone_number`, or `built_in_transfer_to_agent` tools.
/// When switching a webhook tool's `endpoint_method` from POST to GET, its request body parameters must be re-sent with `"query_string"` locations.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolParameterLocation {
    RequestBody,
    QueryString,
    UrlPath,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ToolParameterLocation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RequestBody => serializer.serialize_str("request_body"),
            Self::QueryString => serializer.serialize_str("query_string"),
            Self::UrlPath => serializer.serialize_str("url_path"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ToolParameterLocation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "request_body" => Ok(Self::RequestBody),
            "query_string" => Ok(Self::QueryString),
            "url_path" => Ok(Self::UrlPath),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ToolParameterLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RequestBody => write!(f, "request_body"),
            Self::QueryString => write!(f, "query_string"),
            Self::UrlPath => write!(f, "url_path"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
