---
name: phonic-custom-commands
description: How to author custom commands for the phonic CLI using the co-generated SDK.
---

# Custom Commands for `phonic`

## Overview

The `phonic` CLI supports user-authored custom commands that are
compiled into the binary alongside the auto-generated API commands.
Custom commands get a fully-wired SDK client that inherits the CLI's
auth, retries, TLS, base URL, and global headers — zero configuration required.

## Architecture

```
cli/phonic/custom.rs    ← Your command handlers (protected by .fernignore)
cli/phonic/sdk.rs       ← Generated bridge: client() + block_on()
cli/phonic/main.rs      ← Generated entrypoint (calls custom::register)
phonic-sdk/             ← Co-generated typed SDK crate
phonic-types/           ← Co-generated typed model crate
```

## Adding a Custom Command

### 1. Edit `cli/phonic/custom.rs`

This file is protected by `.fernignore` — `fern generate` will never
overwrite it. Register commands in the `register()` function:

```rust
use phonic_sdk::api::*;

pub fn register(app: CliApp) -> CliApp {
    let app = app.command(
        clap::Command::new("get")
            .about("Returns an agent by name or ID.")
            .arg(clap::Arg::new("nameOrId").required(true))
        ,
        |matches, ctx| {
            let name_or_id = matches.get_one::<String>("nameOrId").unwrap();
            let client = super::sdk::client(ctx);
            let result = super::sdk::block_on(
                client.agents.get(name_or_id),
            )?;
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            Ok(())
        },
    );
    app
}
```

Then build and test:
```bash
cargo build
phonic get <nameOrId>
```

### 2. Available SDK Clients

The `super::sdk::client(ctx)` call returns a `phonic_sdk::api::Client`
with the following sub-clients:

| Field | Type | Description |
|-------|------|-------------|
| `client.agents` | `phonic_sdk::api::AgentsClient` | agents operations |
| `client.tools` | `phonic_sdk::api::ToolsClient` | tools operations |
| `client.extraction_schemas` | `phonic_sdk::api::ExtractionSchemasClient` | extraction_schemas operations |
| `client.voices` | `phonic_sdk::api::VoicesClient` | voices operations |
| `client.workspace` | `phonic_sdk::api::WorkspaceClient` | workspace operations |
| `client.api_keys` | `phonic_sdk::api::ApiKeysClient` | api_keys operations |
| `client.conversation_items` | `phonic_sdk::api::ConversationItemsClient` | conversation_items operations |
| `client.conversations` | `phonic_sdk::api::ConversationsClient` | conversations operations |
| `client.auth` | `phonic_sdk::api::AuthClient` | auth operations |
| `client.tts` | `phonic_sdk::api::TtsClient` | tts operations |
| `client.projects` | `phonic_sdk::api::ProjectsClient` | projects operations |

### 3. Key Patterns

**Get the SDK client** (execution-sharing, fully authenticated):
```rust
let client = super::sdk::client(ctx);
```

**Run an async SDK call from a sync handler:**
```rust
let result = super::sdk::block_on(
    client.some_resource.some_method(args),
)?;
```

**Use typed models for request/response serialization:**
```rust
use phonic_sdk::api::*;
```

### 4. Authentication

Custom commands automatically inherit the CLI's authentication.
The following auth schemes are configured:

- **bearerAuth** (bearer): env `PHONIC_API_KEY`

No manual auth wiring is needed in custom command handlers.

## Regeneration Safety

| File | Regenerated? | Notes |
|------|-------------|-------|
| `cli/phonic/custom.rs` | **No** | Protected by `.fernignore` |
| `cli/phonic/sdk.rs` | Yes | Bridges AppContext → SDK client |
| `cli/phonic/main.rs` | Yes | Calls `custom::register(app)` |
| `phonic-sdk/` | Yes | Co-generated typed SDK crate |
| `phonic-types/` | Yes | Co-generated typed models |

After running `fern generate`, your `custom.rs` is preserved. All
generated code (SDK, types, glue, main.rs) is updated to match the
latest API spec. If the SDK surface changes (renamed methods, new
sub-clients), update your `custom.rs` to match.

## Build & Test

```bash
# Build the CLI (includes custom commands)
cargo build

# Run your custom command
phonic <your-command> [args]

# Run with verbose output for debugging
RUST_LOG=debug phonic <your-command> [args]
```
