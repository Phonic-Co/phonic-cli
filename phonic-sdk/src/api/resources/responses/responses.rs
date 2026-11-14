use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ResponsesClient {
    pub http_client: HttpClient,
}

impl ResponsesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Generates one or more alternative assistant responses for a conversation
    /// you supply inline to simulate Phonic agent behavior.
    ///
    /// This endpoint is stateless, so it does not create a new conversation or
    /// store anything. The request carries the system prompt, a conversation so
    /// far as `input`, and the tools the assistant may call.
    ///
    /// There are two different types of tool configurations:
    ///
    /// - `tool_definitions`: defined inline. Typically if you use LiveKit, this
    /// is where you can pass in your tool definitions.
    /// - `tools`: tools that already exist in Phonic, in the same shape as the
    /// STS WebSocket `config` message. Built-in tools such as
    /// `choose_not_to_respond` are passed by their built-in name or as a
    /// `{type: "built_in", name, tool_config}` object. The only tools stored in
    /// `project` that can be referenced by name are transfer tools (transfer to
    /// phone number and transfer to agent). Stored built-in tool instances
    /// (such as a saved choose not to respond, natural conversation ending or
    /// keypad input tool) cannot be referenced by their saved name; pass the
    /// built-in itself instead. Your context, webhook, MCP and WebSocket tools
    /// cannot be referenced here yet; define those inline as `tool_definitions`.
    ///
    /// Each item in `input` is a user message, an assistant message (with
    /// optional `tool_calls` or an `action`), or a `tool_call_output`. Every
    /// assistant tool call must be followed immediately by the
    /// `tool_call_output` item that carries its result.
    ///
    /// A tool referenced in `tools` resolves to an `action` on the generated
    /// response rather than a tool call - the event a live conversation would
    /// have emitted, without the effect actually being carried out.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn create(
        &self,
        request: &GenerateResponsesRequest,
        options: Option<RequestOptions>,
    ) -> Result<GenerateResponsesResponse, ApiError> {
        let base_url = self
            .http_client
            .config()
            .environment
            .as_ref()
            .map_or(self.http_client.base_url(), |env| env.base_url());
        self.http_client
            .execute_request_with_base_url(
                base_url,
                Method::POST,
                "responses",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
