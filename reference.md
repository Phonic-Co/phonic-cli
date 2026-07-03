# Phonic CLI Reference

Full command reference for `phonic`.

## Commands

- [`phonic agents`](#phonic-agents)
- [`phonic api-keys`](#phonic-api-keys)
- [`phonic auth`](#phonic-auth)
- [`phonic conversation-items`](#phonic-conversation-items)
- [`phonic conversations`](#phonic-conversations)
- [`phonic extraction-schemas`](#phonic-extraction-schemas)
- [`phonic projects`](#phonic-projects)
- [`phonic tools`](#phonic-tools)
- [`phonic tts`](#phonic-tts)
- [`phonic voices`](#phonic-voices)
- [`phonic workspace`](#phonic-workspace)

---

### `phonic agents`

#### `phonic agents add-custom-phone-number`

Adds a custom phone number to an agent. The user must configure their SIP trunk to point to Phonic's SIP server.

`POST /agents/{nameOrId}/custom-phone-numbers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--name-or-id` | `string` | Yes | The name or the ID of the agent. |
| `--project` | `string` | No | The name of the project containing the agent. Only used when `nameOrId` is a name. |
| `--x-sip-address` | `string` | No | SIP address of the user's SIP trunk. Optional, but if provided, all three SIP headers (X-Sip-Address, X-Sip-Auth-Username, X-Sip-Auth-Password) must be provided. When these headers are provided, call transfers from the agent will use the provided SIP details. |
| `--x-sip-auth-username` | `string` | No | SIP auth username. Optional, but if provided, all three SIP headers (X-Sip-Address, X-Sip-Auth-Username, X-Sip-Auth-Password) must be provided. When these headers are provided, call transfers from the agent will use the provided SIP details. |
| `--x-sip-auth-password` | `string` | No | SIP auth password. Optional, but if provided, all three SIP headers (X-Sip-Address, X-Sip-Auth-Username, X-Sip-Auth-Password) must be provided. When these headers are provided, call transfers from the agent will use the provided SIP details. |
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

Only assistant items from ended conversations can be replayed. The
conversation must have an associated agent.


`POST /conversation_items/{id}/replay`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--id` | `string` | Yes | The ID of the conversation item to replay. |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

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

Initiates a SIP outbound call using user-supplied SIP credentials in headers.

`POST /conversations/sip/outbound_call`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--x-sip-address` | `string` | Yes | SIP address of the user's SIP trunk. Required. |
| `--x-sip-auth-username` | `string` | No | SIP auth username, if your provider requires it. |
| `--x-sip-auth-password` | `string` | No | SIP auth password, if your provider requires it. |
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

Returns all available voices for a model.

`GET /voices`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--model` | `merritt` | Yes | The model to get voices for. |

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

