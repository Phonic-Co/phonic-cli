# Phonic CLI Reference

Full command reference for `phonic`.

## Commands

- [`phonic agents`](#phonic-agents)
- [`phonic api-keys`](#phonic-api-keys)
- [`phonic auth`](#phonic-auth)
- [`phonic conversation-items`](#phonic-conversation-items)
- [`phonic conversations`](#phonic-conversations)
- [`phonic external-storage-policies`](#phonic-external-storage-policies)
- [`phonic extraction-schemas`](#phonic-extraction-schemas)
- [`phonic mcp-servers`](#phonic-mcp-servers)
- [`phonic projects`](#phonic-projects)
- [`phonic responses`](#phonic-responses)
- [`phonic tools`](#phonic-tools)
- [`phonic tts`](#phonic-tts)
- [`phonic voices`](#phonic-voices)
- [`phonic workspace`](#phonic-workspace)

---

### `phonic agents`

#### `phonic agents add-custom-phone-number`

Adds a bring-your-own phone number to an agent in the authenticated workspace and project. Use this to connect a phone number and SIP trunk the user is authorized to manage; it does not purchase a number or place a call. Stores the supplied SIP address and credentials for calls and transfers, and saves any configuration endpoint for calls on this number. The user must configure their SIP trunk to route inbound calls to Phonic. A configured endpoint receives requests during call setup; use only an endpoint the user intends Phonic to contact.

`POST /agents/{nameOrId}/custom-phone-numbers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the agent. |
| `--project` | `string` | No | The name of the project containing the agent. Only used when `nameOrId` is a name. |
| `--x-sip-address` | `string` | Yes | SIP address of the user's SIP trunk. Required. All three SIP headers (X-Sip-Address, X-Sip-Auth-Username, X-Sip-Auth-Password) must be provided. They are used for outbound calls and call transfers from the agent. |
| `--x-sip-auth-username` | `string` | Yes | SIP auth username. Required. All three SIP headers (X-Sip-Address, X-Sip-Auth-Username, X-Sip-Auth-Password) must be provided. They are used for outbound calls and call transfers from the agent. |
| `--x-sip-auth-password` | `string` | Yes | SIP auth password. Required. All three SIP headers (X-Sip-Address, X-Sip-Auth-Username, X-Sip-Auth-Password) must be provided. They are used for outbound calls and call transfers from the agent. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic agents create`

Creates a new agent in a project.

`POST /agents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project` | `string` | No | The name of the project to create the agent in. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic agents delete`

Deletes an agent by name or ID.

`DELETE /agents/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the agent to delete. |
| `--project` | `string` | No | The name of the project containing the agent. Only used when `nameOrId` is a name. |

#### `phonic agents delete-custom-phone-number`

Deletes a custom phone number from an agent.

`DELETE /agents/{nameOrId}/custom-phone-numbers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the agent. |
| `--project` | `string` | No | The name of the project containing the agent. Only used when `nameOrId` is a name. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic agents disable`

Disables an agent by name or ID. A disabled agent cannot start conversations or be updated, except to release its phone numbers.

`POST /agents/{nameOrId}/disable`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the agent. |
| `--project` | `string` | No | The name of the project containing the agent. Only used when `nameOrId` is a name. |

#### `phonic agents enable`

Enables a previously disabled agent by name or ID.

`POST /agents/{nameOrId}/enable`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the agent. |
| `--project` | `string` | No | The name of the project containing the agent. Only used when `nameOrId` is a name. |

#### `phonic agents get`

Returns an agent by name or ID.

`GET /agents/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the agent to get. |
| `--project` | `string` | No | The name of the project containing the agent. Only used when `nameOrId` is a name. |

#### `phonic agents list`

Returns all agents in a project.

`GET /agents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project` | `string` | No | The name of the project to list agents for. |

#### `phonic agents update`

Updates an agent by name or ID.

`PATCH /agents/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the agent to update. |
| `--project` | `string` | No | The name of the project containing the agent. Only used when `nameOrId` is a name. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic agents update-phone-number`

Updates a phone number on an agent.

`PATCH /agents/{nameOrId}/phone-numbers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the agent. |
| `--project` | `string` | No | The name of the project containing the agent. Only used when `nameOrId` is a name. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic agents upsert`

Upserts an agent by name. If an agent with the same name already exists, it will be updated. Otherwise, it will be created.

`PUT /agents/upsert`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project` | `string` | No | The name of the project containing the agent. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `phonic api-keys`

#### `phonic api-keys create`

Creates a new API key in the workspace.

`POST /api_keys`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic api-keys delete`

Deletes an API key.

`DELETE /api_keys/{id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the API key to delete. |

#### `phonic api-keys rotate`

Rotates an API key, generating a new secret and invalidating the old one.

`POST /api_keys/{id}/rotate`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the API key to rotate. |

#### `phonic api-keys update`

Updates an API key.

`PATCH /api_keys/{id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the API key to update. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `phonic auth`

#### `phonic auth create-conversation-token`

Creates a short-lived conversation token scoped to a specific agent. Conversation tokens are useful for client-side applications that start a conversation with a single agent without exposing your API key.

`POST /auth/conversation_token`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic auth create-session-token`

Creates a short-lived session token that can be used to authenticate WebSocket connections. Session tokens are useful for client-side applications where you don't want to expose your API key.

`POST /auth/session_token`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `phonic conversation-items`

#### `phonic conversation-items replay`

Returns the alternative response(s) the assistant would have
produced for this conversation turn given changes to the agent system prompt.

Only assistant items from ended conversations can be replayed. Omit the
request body (or omit `system_prompt`) to replay the turn exactly as it
originally ran.


`POST /conversation_items/{id}/replay`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the conversation item to replay. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `phonic conversations`

#### `phonic conversations cancel`

Cancels an active conversation.

`POST /conversations/{id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the conversation to cancel. |

#### `phonic conversations delete`

Deletes a conversation, scheduling its transcripts and audio recordings for deletion. The conversation must have ended.

`DELETE /conversations/{id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the conversation to delete. |

#### `phonic conversations evaluate`

Evaluates a conversation using an evaluation prompt.

`POST /conversations/{id}/evals`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the conversation to evaluate. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic conversations extract-data`

Extracts data from a conversation using a schema.

`POST /conversations/{id}/extractions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the conversation to extract data from. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic conversations get`

Returns a conversation by ID.

`GET /conversations/{id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the conversation to get. |
| `--audio-container` | `wav.gz | wav` | No | Format of the presigned `audio_url` in the response. |

#### `phonic conversations get-analysis`

Returns an analysis of the specified conversation.

`GET /conversations/{id}/analysis`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the conversation to analyze. |

#### `phonic conversations list`

Returns conversations with optional filtering.

`GET /conversations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project` | `string` | No | The name of the project to list conversations for. |
| `--external-id` | `string` | No | Filter by external ID to get a specific conversation. |
| `--duration-min` | `integer` | No | Minimum duration in seconds. |
| `--duration-max` | `integer` | No | Maximum duration in seconds. |
| `--started-at-min` | `string` | No | Minimum start date/time. Valid examples: `2025-04-17`, `2025-04-17T02:48:52.708Z` |
| `--started-at-max` | `string` | No | Maximum start date/time. Valid examples: `2025-04-17`, `2025-04-17T02:48:52.708Z` |
| `--before` | `string` | No | Cursor for backward pagination. Use a conversation ID from `pagination.prev_cursor` to fetch the previous page of conversations. Cannot be used with `after`. |
| `--after` | `string` | No | Cursor for forward pagination. Use a conversation ID from `pagination.next_cursor` to fetch the next page of conversations. Cannot be used with `before`. |
| `--limit` | `integer` | No | Maximum number of conversations to return per page. |
| `--audio-container` | `wav.gz | wav` | No | Format of the presigned `audio_url` in each conversation in the response. |

#### `phonic conversations list-evaluations`

Returns all evaluations for a conversation.

`GET /conversations/{id}/evals`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the conversation to get evaluations for. |

#### `phonic conversations list-extractions`

Returns all extractions for a conversation.

`GET /conversations/{id}/extractions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the conversation to get extractions for. |

#### `phonic conversations outbound-call`

Initiates a call to a given phone number using Phonic's Twilio account.

`POST /conversations/outbound_call`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic conversations replay`

Replays an ended conversation by re-running its recorded audio through an
agent. Requires API key or access token authentication. The conversation must
have audio recordings available and an associated agent (or one specified in
the request body).


`POST /conversations/{id}/replay`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the conversation to replay. |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `phonic conversations sip-outbound-call`

Places an outbound voice-agent phone call through the user's SIP trunk using the supplied SIP credentials, caller ID, destination phone number, and conversation configuration. Use only when the user requests a call to that recipient and is authorized to use the trunk and caller ID. With `dry_run` omitted or false, this contacts the recipient and may incur telephony charges; repeating a successful request may place another call. Set `dry_run` to true to validate setup without placing a call. A dry run does not verify that the recipient can be reached or test call audio.

`POST /conversations/sip/outbound_call`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--x-sip-address` | `string` | Yes | SIP address of the user's SIP trunk. Required. |
| `--x-sip-auth-username` | `string` | No | SIP auth username, if your provider requires it. |
| `--x-sip-auth-password` | `string` | No | SIP auth password, if your provider requires it. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `phonic external-storage-policies`

#### `phonic external-storage-policies create`

Creates a named external storage policy in the authenticated workspace and project, storing the supplied S3-compatible destination and credentials for later artifact delivery. Use this when the user wants agents to export conversation artifacts to a bucket they are authorized to use. Agents referencing this policy send conversation artifacts, which may contain personal data, to that external destination. Creating the policy does not attach it to an agent or export existing conversations.

`POST /external_storage_policies`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project` | `string` | No | The name of the project to create the external storage policy in. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic external-storage-policies delete`

Deletes an external storage policy by name or ID. The policy must not be referenced by any agent.

`DELETE /external_storage_policies/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the external storage policy to delete. |
| `--project` | `string` | No | The name of the project containing the external storage policy. Only used when `nameOrId` is a name. |

#### `phonic external-storage-policies get`

Returns an external storage policy by name or ID.

`GET /external_storage_policies/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the external storage policy to get. |
| `--project` | `string` | No | The name of the project containing the external storage policy. Only used when `nameOrId` is a name. |

#### `phonic external-storage-policies list`

Returns all external storage policies in a project.

`GET /external_storage_policies`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project` | `string` | No | The name of the project to list external storage policies for. |

#### `phonic external-storage-policies update`

Updates the supplied fields of an existing external storage policy by name or ID in the authenticated workspace and project. Use this to change the artifact destination or rotate its credentials. Changes affect subsequent artifact delivery for agents referencing the policy; changing the endpoint or bucket can send conversation artifacts, which may contain personal data, to a different external destination. Confirm the intended destination before updating. Rotate credentials by providing both `access_key_id` and `secret_access_key`. This operation does not move or delete artifacts already delivered.

`PATCH /external_storage_policies/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the external storage policy to update. |
| `--project` | `string` | No | The name of the project containing the external storage policy. Only used when `nameOrId` is a name. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `phonic extraction-schemas`

#### `phonic extraction-schemas create`

Creates a new extraction schema in a project.

`POST /extraction_schemas`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project` | `string` | No | The name of the project to create the extraction schema in. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic extraction-schemas delete`

Deletes an extraction schema by name or ID.

`DELETE /extraction_schemas/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the extraction schema to delete. |
| `--project` | `string` | No | The name of the project containing the extraction schema. Only used when `nameOrId` is a name. |

#### `phonic extraction-schemas get`

Returns an extraction schema by name or ID.

`GET /extraction_schemas/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the extraction schema to get. |
| `--project` | `string` | No | The name of the project containing the extraction schema. Only used when `nameOrId` is a name. |

#### `phonic extraction-schemas list`

Returns all extraction schemas in a project.

`GET /extraction_schemas`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project` | `string` | No | The name of the project to list extraction schemas for. |

#### `phonic extraction-schemas update`

Updates an extraction schema by name or ID.

`PATCH /extraction_schemas/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the extraction schema to update. |
| `--project` | `string` | No | The name of the project containing the extraction schema. Only used when `nameOrId` is a name. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `phonic mcp-servers`

#### `phonic mcp-servers create`

Creates an MCP server in the authenticated workspace and project. Phonic connects to the server, lists its tools and saves them alongside the server, so the server must be reachable from Phonic's network. The supplied headers are sent on every request Phonic makes to the server; their values are stored encrypted and never returned. Creating the server does not attach it to an agent; add its ID to an agent's `mcp_server_ids` to make its tools available.

`POST /mcp_servers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project` | `string` | No | The name of the project to create the MCP server in. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic mcp-servers delete`

Deletes an MCP server by name or ID and removes it from every agent's `mcp_server_ids`.

`DELETE /mcp_servers/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the MCP server to delete. |
| `--project` | `string` | No | The name of the project containing the MCP server. Only used when `nameOrId` is a name. |

#### `phonic mcp-servers get`

Returns an MCP server by name or ID.

`GET /mcp_servers/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the MCP server to get. |
| `--project` | `string` | No | The name of the project containing the MCP server. Only used when `nameOrId` is a name. |

#### `phonic mcp-servers list`

Returns all MCP servers in a project.

`GET /mcp_servers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project` | `string` | No | The name of the project to list MCP servers for. |

#### `phonic mcp-servers update`

Updates the name and/or headers of an existing MCP server by name or ID. Providing `headers` replaces all existing headers. The URL cannot be changed; delete and re-create the server instead.

`PATCH /mcp_servers/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the MCP server to update. |
| `--project` | `string` | No | The name of the project containing the MCP server. Only used when `nameOrId` is a name. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `phonic projects`

#### `phonic projects create`

Creates a new project in a workspace.

`POST /projects`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic projects create-eval-prompt`

Creates a new conversation evaluation prompt for a project.

`POST /projects/{id}/conversation_eval_prompts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the project. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic projects delete`

Deletes a project by name or ID.

`DELETE /projects/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the project to delete. |

#### `phonic projects get`

Returns a project by name or ID.

`GET /projects/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the project to get. |

#### `phonic projects list`

Returns all projects in a workspace.

`GET /projects`

#### `phonic projects list-eval-prompts`

Returns all conversation evaluation prompts for a project.

`GET /projects/{id}/conversation_eval_prompts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the project. |

#### `phonic projects list-evals`

Returns all conversation evaluation results for a project.

`GET /projects/{id}/conversation_evals`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the project. |

#### `phonic projects update`

Updates a project by name or ID.

`PATCH /projects/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the project to update. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `phonic responses`

#### `phonic responses create`

Generates one or more alternative assistant responses for a conversation
you supply inline to simulate Phonic agent behavior.

This endpoint is stateless, so it does not create a new conversation or
store anything. The request carries the system prompt, a conversation so
far as `input`, and the tools the assistant may call.

There are two different types of tool configurations:

- `tool_definitions`: defined inline. Typically if you use LiveKit, this
  is where you can pass in your tool definitions.
- `tools`: predefined tools configured in Phonic, referenced by name, in
  the same shape as the STS WebSocket `config` message. Today this covers
  built-in tools such as `choose_not_to_respond` and transfer tools stored
  in `project`. Your context, webhook, MCP and WebSocket tools cannot be
  referenced here yet; define those inline as `tool_definitions`.

Each item in `input` is a user message, an assistant message (with
optional `tool_calls` or an `action`), or a `tool_call_output`. Every
assistant tool call must be followed immediately by the
`tool_call_output` item that carries its result.

A tool referenced in `tools` resolves to an `action` on the generated
response rather than a tool call - the event a live conversation would
have emitted, without the effect actually being carried out.


`POST /responses`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `phonic tools`

#### `phonic tools create`

Creates a new tool in a project.

`POST /tools`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project` | `string` | No | The name of the project to create the tool in. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `phonic tools delete`

Deletes a tool by name or ID.

`DELETE /tools/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the tool to delete. |
| `--project` | `string` | No | The name of the project containing the tool. Only used when `nameOrId` is a name. |

#### `phonic tools get`

Returns a tool by name or ID.

`GET /tools/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the tool to get. |
| `--project` | `string` | No | The name of the project containing the tool. Only used when `nameOrId` is a name. |

#### `phonic tools list`

Returns all custom tools for the organization.

`GET /tools`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project` | `string` | No | The name of the project to list tools for. |

#### `phonic tools update`

Updates a tool by name or ID.

`PATCH /tools/{nameOrId}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the tool to update. |
| `--project` | `string` | No | The name of the project containing the tool. Only used when `nameOrId` is a name. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `phonic tts`

#### `phonic tts stream`

Streams generated speech audio for the provided text.

`POST /tts/stream`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `phonic voices`

#### `phonic voices get`

Returns a voice by ID.

`GET /voices/{id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the voice to get. |

#### `phonic voices list`

Returns all available voices.

`GET /voices`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--model` | `merritt` | No | Ignored. This parameter is deprecated and can be omitted. |

#### `phonic voices preview`

Generates speech audio for the provided text and returns it as a single base64-encoded string.

`POST /tts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `phonic workspace`

#### `phonic workspace get`

Returns information about the workspace.

`GET /workspace`

#### `phonic workspace update`

Updates the workspace.

`PATCH /workspace`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--json <JSON\|->` | Supply the request body as JSON (or `-` for stdin) |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

