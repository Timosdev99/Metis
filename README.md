# Metis (WIP)

Metis is an AI context manager that lets you switch between coding CLIs without losing context.  
**Status:** in active development — expect rough edges and breaking changes.

## Features
- Initialize a project with `.metis/` storage
- Run a supported CLI and record turns automatically
- Switch CLIs with a generated handoff summary
- Inspect session status/history
- Clean noisy turns from stored sessions

## Supported CLIs
- `claude`
- `codex`
- `gemini`
- `qwen`

## Quick Start
```bash
# In your project
metis init

# Start a session
metis run codex

# Switch to another CLI (after quitting the current one. will add a attach/detach flow later)
metis switch gemini
```

## Commands
- `metis init`  
  Initialize `.metis/` and update `.gitignore`.

- `metis run <cli> [--inject-delay-ms N] [-- ...args]`  
  Launch a CLI with Metis tracking.  
  `--inject-delay-ms` delays handoff injection (default: 500ms).

- `metis switch <cli> [--inject-delay-ms N] [-- ...args]`  
  Summarize the current session, then launch the next CLI with context.

- `metis status`  
  Show session metadata.

- `metis history [--limit N]`  
  Show recent turns.

- `metis add-turn <role> <content> [--cli name]`  
  Manually append a turn.

- `metis clean`  
  Re-sanitize stored turns to remove UI noise.

- `metis adapters`  
  List supported adapters.

- `metis completion <shell>`  
  Generate shell completions.

## Build & Install (Rust)
### Prerequisites
- Rust toolchain (recommended via `rustup`)

### Build
```bash
cargo build
```

### Run from source
```bash
cargo run -- <command>
```

Example:
```bash
cargo run -- run codex
```

### Install as a command
```bash
# From the repo root
cargo install --path .
```

After install, make sure `~/.cargo/bin` is on your PATH:
```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Now you can run:
```bash
metis status
```

### Alternative: Local bin (manual copy)
If you prefer to copy the built binary:
```bash
mkdir -p ~/.local/bin
cp target/release/metis ~/.local/bin/metis
```

Ensure `~/.local/bin` is on your PATH:
```bash
export PATH="$HOME/.local/bin:$PATH"
```

Then:
```bash
metis --version
```

### Alternative: Use the built binary directly
```bash
./target/debug/metis <command>
# or release:
./target/release/metis <command>
```

## Notes
- You must **quit the current CLI** before using `metis switch` but a attach/detach flow will be added later.
- If a CLI isn’t ready to accept the handoff message, increase `--inject-delay-ms`.

