# AI SDK Provider Code Generator

This directory contains the consolidated provider code generator for the AI SDK project.

## Quick Start

### Installation

The generator is a single Python executable: `provider-codegen.py`

**Prerequisites:**

- Python 3.13+
- `uv` package manager

### Running Commands

#### Standard way (with uv):

The script declares its dependencies inline ([PEP 723](https://peps.python.org/pep-0723/)), so `uv run` installs them:

```bash
uv run scripts/provider-codegen.py update
uv run scripts/provider-codegen.py openai-compatible
uv run scripts/provider-codegen.py capabilities
```

#### With shell alias (recommended for developers):

Add this to your `~/.bashrc`, `~/.zshrc`, or equivalent:

```bash
alias provider-codegen='uv run ./scripts/provider-codegen.py'
```

Then reload your shell:

```bash
source ~/.bashrc  # or source ~/.zshrc
```

Now you can simply run:

```bash
provider-codegen update
provider-codegen openai-compatible
provider-codegen capabilities
```

## Commands

### Update Existing Providers (additive)

```bash
# Sync every existing provider's capabilities.rs with models.dev
provider-codegen update

# Preview without writing
provider-codegen update --dry-run

# What CI runs: also write a PR body and a CHANGELOG entry
provider-codegen update --summary summary.md --changelog CHANGELOG.md
```

**Output:** Rewrites `src/providers/{provider}/capabilities.rs` only for providers that changed.

This is the safe, everyday command, and the one the daily CI job runs. It keeps every update releasable as a **patch** version:

- Only providers that already exist in the crate are touched. New providers are never added.
- Existing models are **never removed**, even if models.dev deprecates or delists them, so no constructor disappears from the public API.
- Existing capabilities are **never removed**; new capabilities reported by models.dev are added.
- New, non-deprecated models are added. A model is skipped if its generated type or constructor name would clash with an existing one, a Rust keyword, or a provider method (`builder`, `default`, `new`).
- Each file keeps its header and provider struct name, so hand-written providers (`OpenAI`, `XAI`, …) and manually added models survive.
- Fails (exit code 1) if a capabilities file can't be parsed or the result doesn't pass `cargo fmt`.

Running it twice in a row produces no changes.

### Generate OpenAI-compatible Providers

```bash
# All OpenAI-compatible providers
provider-codegen openai-compatible

# Single provider
provider-codegen openai-compatible deepseek

# With capabilities
provider-codegen openai-compatible --with-capabilities
provider-codegen openai-compatible deepseek -c
```

**Output:** Creates `src/providers/{provider}/mod.rs` (and optionally `capabilities.rs`)

### Generate Capabilities

Unlike `update`, this **regenerates** files from scratch: models that are deprecated or gone from models.dev are removed, which is a breaking change. Use it only for a deliberate, non-patch release.

```bash
# All providers that already exist in the crate
provider-codegen capabilities

# Single provider
provider-codegen capabilities openai

# Multiple providers
provider-codegen capabilities anthropic
```

**Output:** Creates `src/providers/{provider}/capabilities.rs`

## Features

### Atomic File Writing

- All files written together or not at all
- Automatic rollback on errors
- No partial provider updates

### 3-Phase Execution

1. **Content Generation** - Generate all code without touching disk
2. **Atomic Writing** - Write all files together with rollback support
3. **Formatting** - Run `cargo fmt` on generated code

### Error Handling

- Clear error messages with context
- Automatic cleanup of partially written files
- Proper exception propagation

## Architecture

### Consolidated Single-File Executable

The `provider-codegen.py` file contains:

- **Utilities** - Name conversion, project root detection, models.dev API fetching
- **File Writing** - `PendingWrite` dataclass and atomic `batch_write_files()` function
- **Capabilities Generation** - Generate `capabilities.rs` for any provider
- **OpenAI-compatible Generation** - Generate provider modules using macros
- **CLI Commands** - Typer-based command interface

### Key Components

```
provider-codegen.py
├── Utilities (log, fetch_models_dev_json, to_pascal_case, etc.)
├── File Writing (PendingWrite, batch_write_files)
├── Capabilities Generation (generate_capabilities_rs, prepare_provider_capabilities)
├── OpenAI-compatible Generation (generate_openai_compatible_content, prepare_openai_compatible_provider)
├── Additive Update (parse_capabilities_rs, merge_capabilities, prepare_updates)
└── CLI Commands (update, openai_compatible, capabilities commands)
```

## Data Flow

### Content Generation Phase

```
models.dev API JSON
    ↓
fetch_models_dev_json()
    ↓
prepare_openai_compatible_provider() / prepare_provider_capabilities()
    ↓
PendingWrite objects (content + path, no I/O)
```

### Atomic Writing Phase

```
PendingWrite objects
    ↓
batch_write_files()
    ├─ Validate paths
    ├─ Create directories
    ├─ Write files
    └─ Rollback on error
    ↓
List of written file paths
```

## Development

### Updating the Generator

The consolidated file includes all functionality from the modular `scripts/` setup:

- **Utilities** - From `utils.py`
- **File Writing** - From `file_writer.py`
- **Capabilities** - From `capabilities.py`
- **OpenAI-compatible** - From `openai_compatible.py`

To modify the generator:

1. Edit `provider-codegen.py` directly
2. Test with: `python3 -m py_compile provider-codegen.py`
3. Test commands with: `uv run --python 3.13 --with typer provider-codegen.py --help`

### Testing

```bash
# Syntax check
python3 -m py_compile provider-codegen.py

# Help
uv run --python 3.13 --with typer --with requests provider-codegen.py --help

# Dry run (preview changes without writing)
# [Note: dry-run flag available in batch_write_files but not exposed in CLI yet]
```

## Environment

The script automatically:

- Detects project root (looks for `Cargo.toml`)
- Fetches latest data from `https://models.dev/api.json`
- Runs `cargo fmt` after writing files

## Troubleshooting

### `Could not find project root`

Make sure you run the command from within the project directory or one of its subdirectories where `Cargo.toml` exists.

### `Provider not found`

Check available providers with the models.dev API: `curl https://models.dev/api.json | jq 'keys'`

### `Failed to fetch models.dev JSON`

Check your internet connection. The script requires access to `https://models.dev/api.json`

## CI/CD Integration

For GitHub Actions or similar workflows, use the standard invocation:

```yaml
- name: Generate providers
  run: uv run --python 3.13 --with typer --with requests ./scripts/provider-codegen.py openai-compatible --with-capabilities
```

Or with the alias (if available in your shell):

```yaml
- name: Generate providers
  run: |
    alias provider-codegen='uv run --python 3.13 --with typer --with requests ./scripts/provider-codegen.py'
    provider-codegen openai-compatible --with-capabilities
```

## Legacy Scripts

The modular scripts in this directory are maintained for reference and historical purposes:

- `utils.py` - Utility functions (consolidated in `provider-codegen.py`)
- `file_writer.py` - File I/O operations (consolidated in `provider-codegen.py`)
- `capabilities.py` - Capability generation (consolidated in `provider-codegen.py`)
- `openai_compatible.py` - OpenAI-compatible provider generation (consolidated in `provider-codegen.py`)
- `main.py` - CLI entry point (consolidated in `provider-codegen.py`)

These are kept in sync with the consolidated executable but should not be modified directly. All changes should be made to `provider-codegen.py`.
