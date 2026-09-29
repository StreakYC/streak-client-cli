---
name: streak-custom-commands
description: How to author custom commands for the streak CLI using the co-generated SDK.
---

# Custom Commands for `streak`

## Overview

The `streak` CLI supports user-authored custom commands that are
compiled into the binary alongside the auto-generated API commands.
Custom commands get a fully-wired SDK client that inherits the CLI's
auth, retries, TLS, base URL, and global headers — zero configuration required.

## Architecture

```
cli/streak/custom.rs    ← Your command handlers (protected by .fernignore)
cli/streak/sdk.rs       ← Generated bridge: client() + block_on()
cli/streak/main.rs      ← Generated entrypoint (calls custom::register)
streak-sdk/             ← Co-generated typed SDK crate
streak-types/           ← Co-generated typed model crate
```

## Adding a Custom Command

### 1. Edit `cli/streak/custom.rs`

This file is protected by `.fernignore` — `fern generate` will never
overwrite it. Register commands in the `register()` function:

```rust
use streak_sdk::api::*;

pub fn register(app: CliApp) -> CliApp {
    let app = app.command(
        clap::Command::new("get-box-markdown")
            .about("Get box markdown")
            .arg(clap::Arg::new("boxKey").required(true))
        ,
        |matches, ctx| {
            let box_key = matches.get_one::<String>("boxKey").unwrap();
            let client = super::sdk::client(ctx);
            let result = super::sdk::block_on(
                client.boxes.get_box_markdown(box_key),
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
streak get-box-markdown <boxKey>
```

### 2. Available SDK Clients

The `super::sdk::client(ctx)` call returns a `streak_sdk::api::Client`
with the following sub-clients:

| Field | Type | Description |
|-------|------|-------------|
| `client.api_keys` | `streak_sdk::api::ApiKeysClient` | api_keys operations |
| `client.boxes` | `streak_sdk::api::BoxesClient` | boxes operations |
| `client.pipelines` | `streak_sdk::api::PipelinesClient` | pipelines operations |
| `client.users` | `streak_sdk::api::UsersClient` | users operations |
| `client.comments` | `streak_sdk::api::CommentsClient` | comments operations |
| `client.meetings` | `streak_sdk::api::MeetingsClient` | meetings operations |
| `client.tasks` | `streak_sdk::api::TasksClient` | tasks operations |
| `client.contacts` | `streak_sdk::api::ContactsClient` | contacts operations |
| `client.organizations` | `streak_sdk::api::OrganizationsClient` | organizations operations |
| `client.pipeline_stages` | `streak_sdk::api::PipelineStagesClient` | pipeline_stages operations |
| `client.teams` | `streak_sdk::api::TeamsClient` | teams operations |

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
use streak_sdk::api::*;
```

### 4. Authentication

Custom commands automatically inherit the CLI's authentication.
The following auth schemes are configured:

- **bearerAuth** (bearer): env `STREAK_TOKEN`

No manual auth wiring is needed in custom command handlers.

## Regeneration Safety

| File | Regenerated? | Notes |
|------|-------------|-------|
| `cli/streak/custom.rs` | **No** | Protected by `.fernignore` |
| `cli/streak/sdk.rs` | Yes | Bridges AppContext → SDK client |
| `cli/streak/main.rs` | Yes | Calls `custom::register(app)` |
| `streak-sdk/` | Yes | Co-generated typed SDK crate |
| `streak-types/` | Yes | Co-generated typed models |

After running `fern generate`, your `custom.rs` is preserved. All
generated code (SDK, types, glue, main.rs) is updated to match the
latest API spec. If the SDK surface changes (renamed methods, new
sub-clients), update your `custom.rs` to match.

## Build & Test

```bash
# Build the CLI (includes custom commands)
cargo build

# Run your custom command
streak <your-command> [args]

# Run with verbose output for debugging
RUST_LOG=debug streak <your-command> [args]
```
