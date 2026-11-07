#!/usr/bin/env python3
# /// script
# requires-python = ">=3.13"
# dependencies = ["typer", "requests", "tomlkit"]
# ///
"""
AI SDK Provider Code Generator

This single-file executable generates Rust provider and capability code from
models.dev API data. `uv run` installs the dependencies declared above.

Usage:
    uv run scripts/provider-codegen.py update
    uv run scripts/provider-codegen.py openai-compatible
    uv run scripts/provider-codegen.py openai-compatible deepseek --with-capabilities
    uv run scripts/provider-codegen.py capabilities
    uv run scripts/provider-codegen.py capabilities openai
"""

import re
import subprocess
import requests
from pathlib import Path
from typing import Any, Optional, Literal, Annotated
from dataclasses import dataclass
import tomlkit
import typer

# ============================================================================
# UTILITIES
# ============================================================================


def log(message: str):
    """Log progress messages."""
    print(f"[INFO] {message}")


def fetch_models_dev_json() -> dict[str, Any]:
    """
    Fetch models.dev API JSON.

    Returns:
        Dictionary containing all provider and model data from models.dev
    """
    log("Fetching models.dev API data...")
    response = requests.get("https://models.dev/api.json")
    if response.status_code != 200:
        raise Exception(f"Failed to fetch models.dev JSON: {response.status_code}")

    log("Successfully fetched models.dev data")
    return response.json()


def to_pascal_case(identifier: str) -> str:
    """
    Convert any identifier to PascalCase for Rust struct/type names.

    Handles edge cases like:
    - 302ai -> Ai302 (identifiers can't start with digits)
    - open-router -> OpenRouter
    - gpt-3.5-turbo -> Gpt35Turbo
    - some_provider -> SomeProvider

    Args:
        identifier: Any identifier (provider ID, model name, etc.)

    Returns:
        PascalCase identifier suitable for Rust
    """
    # Extract leading digits if any
    match = re.match(r"^(\d+)(.*)$", identifier)
    if match:
        digits, rest = match.groups()
        # Move digits to end: "302ai" -> "ai302"
        identifier = rest + digits

    # Replace any character not supported by Rust identifiers with underscores
    # Rust identifiers can only contain: a-z, A-Z, 0-9, _
    cleaned = re.sub(r"[^a-zA-Z0-9_]", "_", identifier)

    # Convert to PascalCase
    return "".join(word.capitalize() for word in cleaned.split("_") if word)


def provider_id_to_snake_case(provider_id: str) -> str:
    """
    Convert provider ID to snake_case for module paths.

    Handles edge cases like:
    - 302ai -> ai_302 (module names can't start with digits)
    - open-router -> open_router
    - deep-seek -> deep_seek
    - 123-test-provider -> test_provider_123

    Args:
        provider_id: Provider identifier (kebab-case)

    Returns:
        snake_case identifier suitable for Rust module paths
    """
    # Extract leading digits if any
    match = re.match(r"^(\d+)(.*)$", provider_id)
    if match:
        digits, rest = match.groups()
        # Move digits to end with underscore: "302ai" -> "ai_302"
        provider_id = rest + "_" + digits

    # Replace hyphens (and any other non-identifier character, e.g. "wafer.ai")
    # with underscores and clean up
    result = re.sub(r"[^a-zA-Z0-9_]", "_", provider_id)

    # Remove leading/trailing underscores and consecutive underscores
    result = re.sub(r"_+", "_", result).strip("_")

    return result


def to_constructor_name(identifier: str) -> str:
    """
    Convert any identifier to snake_case constructor name.

    Handles:
    - gpt-3.5-turbo -> gpt_3_5_turbo
    - claude-3-opus -> claude_3_opus
    - 302ai -> ai_302 (identifiers can't start with digits)

    Args:
        identifier: Any identifier (provider ID, model name, etc.)

    Returns:
        snake_case constructor name
    """
    # Extract leading digits if any
    match = re.match(r"^(\d+)(.*)$", identifier)
    if match:
        digits, rest = match.groups()
        # Move digits to end: "302ai" -> "ai302"
        identifier = rest + digits

    # Replace any character not supported by Rust identifiers with underscores
    # Rust identifiers can only contain: a-z, A-Z, 0-9, _
    cleaned = re.sub(r"[^a-zA-Z0-9_]", "_", identifier)

    # Remove consecutive underscores and trailing underscores
    cleaned = re.sub(r"_+", "_", cleaned).strip("_")
    return cleaned.lower()


def get_project_root() -> Path:
    """
    Get the project root directory (where Cargo.toml is located).

    Returns:
        Path to project root

    Raises:
        RuntimeError: If Cargo.toml is not found
    """
    root = Path(__file__).resolve().parent.parent
    if not (root / "Cargo.toml").exists():
        raise RuntimeError("Could not find project root (missing Cargo.toml)")
    return root


def filter_openai_compatible_providers(
    all_providers: dict[str, Any],
) -> dict[str, Any]:
    """
    Filter providers to only include OpenAI-compatible ones.

    OpenAI-compatible providers are identified by:
    - npm: "@ai-sdk/openai-compatible"

    Args:
        all_providers: All providers from models.dev API

    Returns:
        Dictionary of OpenAI-compatible providers
    """
    return {
        provider_id: provider_data
        for provider_id, provider_data in all_providers.items()
        if provider_data.get("npm") == "@ai-sdk/openai-compatible"
        and provider_id != "privatemode-ai"  # Excluded for known reasons
    }


def get_model_capabilities(model_data: dict[str, Any]) -> list[str]:
    """
    Extract capabilities from model data based on schema.

    Args:
        model_data: Model configuration from API JSON

    Returns:
        Sorted list of capability trait names
    """
    capabilities = []

    # Direct boolean capability fields
    if model_data.get("tool_call", False):
        capabilities.append("ToolCallSupport")

    if model_data.get("reasoning", False):
        capabilities.append("ReasoningSupport")

    if model_data.get("structured_output", False):
        capabilities.append("StructuredOutputSupport")

    # Modalities-based capabilities
    modalities = model_data.get("modalities", {})
    input_modalities = modalities.get("input", [])
    output_modalities = modalities.get("output", [])

    # Input capabilities
    if "text" in input_modalities:
        capabilities.append("TextInputSupport")

    if "audio" in input_modalities:
        capabilities.append("AudioInputSupport")

    # Image input: either attachment=true OR "image" in modalities.input
    if "image" in input_modalities or model_data.get("attachment", False):
        capabilities.append("ImageInputSupport")

    if "video" in input_modalities:
        capabilities.append("VideoInputSupport")

    # Output capabilities
    if "text" in output_modalities:
        capabilities.append("TextOutputSupport")

    if "audio" in output_modalities:
        capabilities.append("AudioOutputSupport")

    if "image" in output_modalities:
        capabilities.append("ImageOutputSupport")

    if "video" in output_modalities:
        capabilities.append("VideoOutputSupport")

    # Remove duplicates and sort
    return sorted(list(set(capabilities)))


# ============================================================================
# FILE WRITING
# ============================================================================


def resolve_provider_dir(provider_id: str, root: Path) -> Path:
    """
    Resolve the on-disk directory for a models.dev provider ID.

    Prefers an existing directory, so hand-written providers stored in
    snake_case (e.g. 'amazon_bedrock' for models.dev 'amazon-bedrock') are
    updated in place instead of getting a duplicate kebab-case directory.

    Args:
        provider_id: Provider identifier (kebab-case from models.dev)
        root: Project root directory

    Returns:
        Path to the provider directory (may not exist yet)
    """
    providers_dir = root / "src" / "providers"
    for candidate in (provider_id, provider_id.replace("-", "_")):
        if (providers_dir / candidate).is_dir():
            return providers_dir / candidate
    return providers_dir / provider_id


@dataclass
class PendingWrite:
    """Represents a file to be written."""

    path: Path
    content: str
    file: Literal["mod.rs", "capabilities.rs"]


def create_pending_write(
    provider_id: str,
    content: str,
    file_type: Literal["mod", "capabilities"],
    root: Path,
) -> PendingWrite:
    """
    Helper to create PendingWrite with computed path.

    Args:
        provider_id: The provider identifier (e.g., 'deepseek')
        content: The file content to write
        file_type: Either 'mod' or 'capabilities'
        root: The root directory (typically repo root)

    Returns:
        PendingWrite object with computed path
    """
    provider_dir = resolve_provider_dir(provider_id, root)

    if file_type == "mod":
        file_path = provider_dir / "mod.rs"
        file_type_full = "mod.rs"
    elif file_type == "capabilities":
        file_path = provider_dir / "capabilities.rs"
        file_type_full = "capabilities.rs"
    else:
        raise ValueError(f"Unknown file_type: {file_type}")

    return PendingWrite(path=file_path, content=content, file=file_type_full)


def batch_write_files(
    pending_writes: list[PendingWrite], dry_run: bool = False
) -> list[Path]:
    """
    Write all files atomically.

    Strategy:
    1. Validate all paths are writable
    2. Create all parent directories
    3. Write all files
    4. On error: attempt rollback of written files
    5. Return list of successfully written paths

    Args:
        pending_writes: List of PendingWrite objects to write
        dry_run: If True, only log what would be written without writing files

    Returns:
        List of successfully written file paths

    Raises:
        Exception: If any write operation fails (after attempting rollback)
    """
    if not pending_writes:
        log("No files to write")
        return []

    if dry_run:
        log(f"[DRY RUN] Would write {len(pending_writes)} files:")
        for pw in pending_writes:
            log(f"  - {pw.path} ({pw.file})")
        return []

    written_files: list[Path] = []

    try:
        # Phase 1: Validate and log what will be overwritten
        for pw in pending_writes:
            if pw.path.exists():
                log(f"Will overwrite: {pw.path}")

        # Phase 2: Create all parent directories
        for pw in pending_writes:
            pw.path.parent.mkdir(parents=True, exist_ok=True)

        # Phase 3: Write all files
        for pw in pending_writes:
            with open(pw.path, "w", encoding="utf-8") as f:
                f.write(pw.content)
            written_files.append(pw.path)
            log(f"Wrote: {pw.path}")

        return written_files

    except Exception as e:
        # Rollback: delete partially written files
        log(f"Error during file writing: {e}")
        log(f"Rolling back {len(written_files)} written files...")

        for path in written_files:
            try:
                path.unlink()
                log(f"Rolled back: {path}")
            except Exception as rollback_error:
                log(f"Failed to rollback {path}: {rollback_error}")

        raise  # Re-raise to propagate to CLI


# ============================================================================
# CARGO.TOML AND MOD.RS UPDATES
# ============================================================================

# Providers that are manually managed in src/providers/mod.rs
# (declared above the // [codegen] block). These are excluded from codegen.
MANUALLY_MANAGED_MODULES = {
    "openai",
    "openai_compatible",
    "openai_chat_completions",
    "anthropic",
    "groq",
    "google",
    "vercel",
    "openrouter",
    "mistral",
    "amazon_bedrock",
    "togetherai",
    "xai",
}


def update_cargo_toml(provider_ids: list[str], root: Path | None = None):
    """
    Add feature entries for new providers to Cargo.toml (additive only).

    For each provider_id, this ensures:
    1. A feature entry `provider-id = ["openaichatcompletions"]` exists
    2. The provider is listed in the `full` feature array

    Uses tomlkit for style-preserving TOML manipulation.

    Args:
        provider_ids: List of provider identifiers (kebab-case from models.dev)
        root: Optional project root (defaults to auto-detected root)
    """
    if root is None:
        root = get_project_root()

    cargo_path = root / "Cargo.toml"
    with open(cargo_path, "r", encoding="utf-8") as f:
        doc = tomlkit.load(f)

    features = doc["features"]
    full_feature = features["full"]
    modified = False

    for provider_id in sorted(provider_ids):
        # Add individual feature entry if not present
        if provider_id not in features:
            features[provider_id] = ["openaichatcompletions"]
            log(f"Added feature '{provider_id}' to Cargo.toml")
            modified = True

        # Add to 'full' feature list if not present
        if provider_id not in full_feature:
            full_feature.append(provider_id)
            log(f"Added '{provider_id}' to 'full' feature list")
            modified = True

    if modified:
        with open(cargo_path, "w", encoding="utf-8") as f:
            tomlkit.dump(doc, f)
        log("Updated Cargo.toml")
    else:
        log("Cargo.toml already up to date")


def get_codegen_provider_dirs(root: Path) -> list[str]:
    """
    Scan src/providers/ for directories that should be managed by codegen.

    Excludes manually-managed providers and non-directory entries.

    Args:
        root: Project root directory

    Returns:
        Sorted list of provider directory names (as they appear on disk)
    """
    providers_dir = root / "src" / "providers"
    codegen_dirs = []

    for entry in sorted(providers_dir.iterdir()):
        if not entry.is_dir():
            continue
        dir_name = entry.name
        # Normalize to check against manually-managed set
        # (dirs may use hyphens or underscores)
        normalized = dir_name.replace("-", "_")
        if normalized in MANUALLY_MANAGED_MODULES:
            continue
        codegen_dirs.append(dir_name)

    return codegen_dirs


def update_providers_mod_rs(root: Path | None = None):
    """
    Regenerate the codegen block in src/providers/mod.rs.

    Scans all provider directories and generates module declarations
    and re-exports between the // [codegen] and // [end-codegen] markers.

    For providers with hyphens in the directory name, a #[path] attribute
    is used since Rust module names cannot contain hyphens.

    Args:
        root: Optional project root (defaults to auto-detected root)
    """
    if root is None:
        root = get_project_root()

    mod_rs_path = root / "src" / "providers" / "mod.rs"
    content = mod_rs_path.read_text(encoding="utf-8")

    # Find the codegen markers
    start_marker = "// [codegen]"
    end_marker = "// [end-codegen]"

    start_idx = content.find(start_marker)
    end_idx = content.find(end_marker)

    if start_idx == -1 or end_idx == -1:
        raise RuntimeError(
            f"Could not find codegen markers in {mod_rs_path}. "
            f"Expected '{start_marker}' and '{end_marker}'"
        )

    # Get all codegen-managed provider directories
    provider_dirs = get_codegen_provider_dirs(root)

    # Generate the codegen block content
    lines = []
    for dir_name in provider_dirs:
        module_name = provider_id_to_snake_case(dir_name)
        struct_name = to_pascal_case(dir_name)
        # Feature names use the provider ID as-is (kebab-case)
        feature_name = dir_name
        # Use #[path] when directory name differs from module name
        # (e.g., hyphens in dir name, or leading digits like "302ai" -> "ai_302")
        needs_path_attr = dir_name != module_name

        lines.append(f'#[cfg(feature = "{feature_name}")]')
        if needs_path_attr:
            lines.append(f'#[path = "{dir_name}/mod.rs"]')
        lines.append(f"pub mod {module_name};")
        lines.append(f'#[cfg(feature = "{feature_name}")]')
        lines.append(f"pub use {module_name}::{struct_name};")
        lines.append("")

    # Build the new content between markers
    codegen_content = "\n".join(lines)
    new_block = f"{start_marker}\n{codegen_content}{end_marker}"

    # Replace the codegen block
    new_content = content[:start_idx] + new_block + content[end_idx + len(end_marker) :]

    mod_rs_path.write_text(new_content, encoding="utf-8")
    log(f"Updated {mod_rs_path} with {len(provider_dirs)} codegen providers")


# ============================================================================
# CAPABILITIES GENERATION
# ============================================================================


def get_model_display_name(model_id: str, model_data: dict[str, Any]) -> str:
    """
    Get the display name for a model.

    Args:
        model_id: Model identifier from API
        model_data: Model configuration data

    Returns:
        Human-readable display name
    """
    return model_data.get("name", model_id)


def get_model_constructor_name(model_id: str, folder_prefix: str | None = None) -> str:
    """
    Get the constructor name for a model with optional folder prefix.

    Handles multi-segment prefixes (e.g., "Pro/deepseek-ai") by converting
    each segment to snake_case and joining with underscores.

    Args:
        model_id: Model identifier
        folder_prefix: Optional folder prefix for nested models (may contain "/")

    Returns:
        Constructor name in snake_case
    """
    if folder_prefix:
        prefix_parts = folder_prefix.split("/")
        prefix = "_".join(to_constructor_name(p) for p in prefix_parts)
        return prefix + "_" + to_constructor_name(model_id)
    return to_constructor_name(model_id)


def get_model_type_name(model_id: str, folder_prefix: str | None = None) -> str:
    """
    Get the type name (PascalCase) for a model with optional folder prefix.

    Handles multi-segment prefixes (e.g., "Pro/deepseek-ai") by converting
    each segment to PascalCase and concatenating them.

    Args:
        model_id: Model identifier
        folder_prefix: Optional folder prefix for nested models (may contain "/")

    Returns:
        Type name in PascalCase
    """
    if folder_prefix:
        prefix_parts = folder_prefix.split("/")
        prefix = "".join(to_pascal_case(p) for p in prefix_parts)
        return prefix + to_pascal_case(model_id)
    return to_pascal_case(model_id)


def parse_model_id(model_id: str) -> tuple[str, str | None]:
    """
    Parse model ID to extract base name and folder prefix if present.

    Args:
        model_id: Model identifier (may contain "/" for nested models)

    Returns:
        Tuple of (base_name, folder_prefix)
    """
    if "/" in model_id:
        parts = model_id.split("/")
        if len(parts) == 2:
            return parts[1], parts[0]
        elif len(parts) > 2:
            # Join all prefix segments to preserve uniqueness
            # e.g., "Pro/deepseek-ai/DeepSeek-R1" -> base="DeepSeek-R1", prefix="Pro/deepseek-ai"
            return parts[-1], "/".join(parts[:-1])

    return model_id, None


def generate_capabilities_rs(
    provider_id: str,
    models: dict[str, Any],
    existing: Optional["CapabilitiesFile"] = None,
) -> str:
    """
    Generate the complete capabilities.rs content.

    Args:
        provider_id: Provider identifier (kebab-case)
        models: Dictionary of model configurations from API
        existing: Parsed current capabilities.rs, if any. Its header and
            provider struct name are reused, since hand-written providers
            don't follow the generated naming (e.g. `OpenAI`, not `Openai`).

    Returns:
        Generated Rust code as string
    """
    provider_struct_name = to_pascal_case(provider_id)
    provider_module = provider_id_to_snake_case(provider_id)

    if existing is not None:
        provider_struct_name = existing.provider
        lines = existing.header.rstrip("\n").split("\n") + [""]
    else:
        lines = [
            f"//! Capabilities for {provider_module} models.",
            "//!",
            f"//! This module defines model types and their capabilities for {
                provider_module
            } providers.",
            "//! Users can implement additional traits on custom models.",
            "",
            "use crate::core::capabilities::*;",
            "use crate::model_capabilities;",
            f"use crate::providers::{provider_module}::{provider_struct_name};",
            "",
        ]

    lines += [
        "model_capabilities! {",
        f"    provider: {provider_struct_name},",
        "    models: {",
    ]

    # Generate model entries
    # Skip deprecated models
    active_models = {
        model_id: model_data
        for model_id, model_data in models.items()
        if model_data.get("status") != "deprecated"
    }

    # Track seen type names to skip duplicates (e.g., "claude-3-5-haiku" and
    # "claude-3.5-haiku" both produce "AnthropicClaude35Haiku")
    seen_type_names: set[str] = set()

    for model_id, model_data in sorted(active_models.items()):
        # Parse model ID for folder structure
        base_name, folder_prefix = parse_model_id(model_id)

        # Get model components
        model_type_name = get_model_type_name(base_name, folder_prefix)

        # Skip duplicate PascalCase type names — these are the same model
        # listed under different naming conventions (e.g., dots vs hyphens)
        if model_type_name in seen_type_names:
            log(
                f"Skipping duplicate type name '{model_type_name}' for model '{model_id}'"
            )
            continue
        seen_type_names.add(model_type_name)

        model_name = model_id  # Use full model ID as the model name
        constructor_name = get_model_constructor_name(base_name, folder_prefix)
        display_name = get_model_display_name(model_id, model_data)
        capabilities = get_model_capabilities(model_data)

        lines.extend(
            render_model_entry(
                ModelEntry(
                    type_name=model_type_name,
                    model_name=model_name,
                    constructor_name=constructor_name,
                    display_name=display_name,
                    capabilities=capabilities,
                )
            )
        )

    lines.extend(
        [
            "    }",
            "}",
        ]
    )

    return "\n".join(lines) + "\n"


def generate_provider_capabilities_content(
    provider_id: str, provider_data: dict[str, Any]
) -> str | None:
    """
    Generate capabilities.rs content without writing to disk.

    Args:
        provider_id: Provider identifier (kebab-case)
        provider_data: Provider configuration from API

    Returns:
        Generated Rust code as string, or None if no models found
    """
    models = provider_data.get("models", {})
    if not models:
        log(f"Warning: No models found for provider '{provider_id}'")
        return None

    log(f"Preparing capabilities for '{provider_id}' with {len(models)} models")
    existing_path = resolve_provider_dir(provider_id, get_project_root()) / (
        "capabilities.rs"
    )
    existing = (
        parse_capabilities_rs(existing_path.read_text(encoding="utf-8"))
        if existing_path.exists()
        else None
    )
    return generate_capabilities_rs(provider_id, models, existing)


def prepare_provider_capabilities(
    provider_id: str, provider_data: dict[str, Any], root: Path | None = None
) -> PendingWrite | None:
    """
    Prepare capabilities.rs for writing (no I/O).

    Args:
        provider_id: Provider identifier (kebab-case)
        provider_data: Provider configuration from API
        root: Optional project root (defaults to auto-detected root)

    Returns:
        PendingWrite object or None if no models found
    """
    if root is None:
        root = get_project_root()

    content = generate_provider_capabilities_content(provider_id, provider_data)
    if content is None:
        return None

    return create_pending_write(provider_id, content, "capabilities", root)


def prepare_all_capabilities(
    all_providers: dict[str, Any],
) -> list[PendingWrite]:
    """
    Prepare capabilities for every provider that already exists in the crate (no I/O).

    Providers that only exist on models.dev are skipped: a capabilities.rs
    without a mod.rs, Cargo feature and module declaration would not compile.
    Use `openai-compatible <id> -c` to add a new provider.

    Args:
        all_providers: All providers from models.dev API

    Returns:
        List of PendingWrite objects
    """
    root = get_project_root()
    pending_writes = []

    for provider_id, provider_data in all_providers.items():
        if not (resolve_provider_dir(provider_id, root) / "mod.rs").exists():
            continue
        try:
            pending_write = prepare_provider_capabilities(
                provider_id, provider_data, root
            )
            if pending_write:
                pending_writes.append(pending_write)
        except Exception as e:
            log(f"Error preparing capabilities for '{provider_id}': {e}")

    return pending_writes


# ============================================================================
# CAPABILITIES PARSING AND ADDITIVE UPDATE
# ============================================================================

# Constructor names that would clash with Rust keywords or with inherent
# methods every provider already has.
RESERVED_CONSTRUCTOR_NAMES = {
    "as", "async", "await", "box", "break", "const", "continue", "crate", "dyn",
    "else", "enum", "extern", "false", "fn", "for", "gen", "if", "impl", "in",
    "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self",
    "static", "struct", "super", "trait", "true", "try", "type", "unsafe", "use",
    "where", "while", "yield", "builder", "default", "new",
}

# Type names already in scope inside every capabilities.rs.
RESERVED_TYPE_NAMES = {"DynamicModel", "ModelName", "Self"}

RUST_IDENT_RE = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")

MODEL_ENTRY_RE = re.compile(
    r"(?P<type_name>\w+)\s*\{\s*"
    r'model_name:\s*"(?P<model_name>(?:[^"\\]|\\.)*)"\s*,\s*'
    r"constructor_name:\s*(?P<constructor_name>\w+)\s*,\s*"
    r'display_name:\s*"(?P<display_name>(?:[^"\\]|\\.)*)"\s*,\s*'
    r"capabilities:\s*\[(?P<capabilities>[^\]]*)\]\s*,?\s*\}"
)


@dataclass
class ModelEntry:
    """One model inside a `model_capabilities!` invocation."""

    type_name: str
    model_name: str
    constructor_name: str
    display_name: str
    capabilities: list[str]


@dataclass
class CapabilitiesFile:
    """A parsed capabilities.rs file."""

    header: str  # everything before `model_capabilities! {`
    provider: str
    entries: list[ModelEntry]


def rust_str_escape(value: str) -> str:
    """Escape a value for use inside a Rust string literal."""
    return value.replace("\\", "\\\\").replace('"', '\\"')


def rust_str_unescape(value: str) -> str:
    """Reverse `rust_str_escape`."""
    return re.sub(r"\\(.)", r"\1", value)


def render_model_entry(entry: ModelEntry) -> list[str]:
    """Render a model entry as `model_capabilities!` lines."""
    return [
        f"        {entry.type_name} {{",
        f'            model_name: "{rust_str_escape(entry.model_name)}",',
        f"            constructor_name: {entry.constructor_name},",
        f'            display_name: "{rust_str_escape(entry.display_name)}",',
        f"            capabilities: [{', '.join(entry.capabilities)}]",
        "        },",
    ]


def render_capabilities_rs(parsed: CapabilitiesFile) -> str:
    """Render a parsed capabilities file back to Rust, models sorted by model name."""
    lines = parsed.header.rstrip("\n").split("\n") + [
        "",
        "model_capabilities! {",
        f"    provider: {parsed.provider},",
        "    models: {",
    ]
    for entry in sorted(parsed.entries, key=lambda e: e.model_name):
        lines.extend(render_model_entry(entry))
    lines.extend(["    }", "}"])
    return "\n".join(lines) + "\n"


def parse_capabilities_rs(content: str) -> CapabilitiesFile:
    """
    Parse a capabilities.rs file made of a single `model_capabilities!` call.

    Raises:
        ValueError: If the file does not have the expected shape.
    """
    macro_idx = content.find("model_capabilities! {")
    if macro_idx == -1:
        raise ValueError("no `model_capabilities! {` invocation found")

    provider_match = re.search(r"provider:\s*(\w+)", content[macro_idx:])
    if not provider_match:
        raise ValueError("no `provider:` field found")

    entries = [
        ModelEntry(
            type_name=m["type_name"],
            model_name=rust_str_unescape(m["model_name"]),
            constructor_name=m["constructor_name"],
            display_name=rust_str_unescape(m["display_name"]),
            capabilities=[c.strip() for c in m["capabilities"].split(",") if c.strip()],
        )
        for m in MODEL_ENTRY_RE.finditer(content, macro_idx)
    ]

    # Every `Name {` inside the macro body must have parsed, otherwise a
    # rewrite would silently drop models.
    body = content[macro_idx + len("model_capabilities! {") :]
    body = re.sub(r'"(?:[^"\\]|\\.)*"', '""', body)  # ignore braces in strings
    opened = len(re.findall(r"\b\w+\s*\{", body))  # `models: {` has a colon, no match
    if opened != len(entries):
        raise ValueError(
            f"parsed {len(entries)} models but found {opened} entries; "
            "file was likely edited by hand in an unexpected format"
        )

    return CapabilitiesFile(
        header=content[:macro_idx],
        provider=provider_match.group(1),
        entries=entries,
    )


@dataclass
class ProviderUpdate:
    """Result of additively merging models.dev data into one capabilities.rs."""

    dir_name: str
    path: Path
    content: str
    added_models: list[str]
    added_capabilities: list[tuple[str, list[str]]]
    not_upstream: list[str]


def merge_capabilities(
    dir_name: str,
    path: Path,
    current: CapabilitiesFile,
    upstream_models: dict[str, Any],
) -> ProviderUpdate | None:
    """
    Additively merge models.dev data into an existing capabilities file.

    Rules (keep every update semver-compatible, i.e. releasable as a patch):
    - existing models are never removed, even if deprecated or gone upstream
    - existing capabilities are never removed; new upstream ones are added
    - new, non-deprecated upstream models are added
    - a new model is skipped if its type or constructor name would clash

    Returns:
        A ProviderUpdate, or None if nothing changed.
    """
    entries = list(current.entries)
    by_model = {e.model_name: e for e in entries}
    type_names = {e.type_name for e in entries}
    constructor_names = {e.constructor_name for e in entries}

    not_upstream = sorted(
        m
        for m in by_model
        if m not in upstream_models
        or upstream_models[m].get("status") == "deprecated"
    )

    added_models: list[str] = []
    added_capabilities: list[tuple[str, list[str]]] = []

    for model_id, model_data in sorted(upstream_models.items()):
        capabilities = get_model_capabilities(model_data)

        if model_id in by_model:
            entry = by_model[model_id]
            new_caps = sorted(set(capabilities) - set(entry.capabilities))
            if new_caps:
                entry.capabilities = sorted(set(entry.capabilities) | set(new_caps))
                added_capabilities.append((model_id, new_caps))
            continue

        if model_data.get("status") == "deprecated":
            continue

        base_name, folder_prefix = parse_model_id(model_id)
        type_name = get_model_type_name(base_name, folder_prefix)
        constructor_name = get_model_constructor_name(base_name, folder_prefix)

        if (
            not RUST_IDENT_RE.match(type_name)
            or not RUST_IDENT_RE.match(constructor_name)
            or type_name in RESERVED_TYPE_NAMES
            or constructor_name in RESERVED_CONSTRUCTOR_NAMES
            or type_name in type_names
            or constructor_name in constructor_names
        ):
            log(f"Skipping '{dir_name}/{model_id}': name clash ({type_name})")
            continue

        entry = ModelEntry(
            type_name=type_name,
            model_name=model_id,
            constructor_name=constructor_name,
            display_name=get_model_display_name(model_id, model_data),
            capabilities=capabilities,
        )
        entries.append(entry)
        by_model[model_id] = entry
        type_names.add(type_name)
        constructor_names.add(constructor_name)
        added_models.append(model_id)

    if not added_models and not added_capabilities:
        return None

    merged = CapabilitiesFile(current.header, current.provider, entries)
    return ProviderUpdate(
        dir_name=dir_name,
        path=path,
        content=render_capabilities_rs(merged),
        added_models=added_models,
        added_capabilities=added_capabilities,
        not_upstream=not_upstream,
    )


def resolve_models_dev_id(dir_name: str, all_providers: dict[str, Any]) -> str | None:
    """Map a provider directory name to its models.dev provider ID."""
    for candidate in (dir_name, dir_name.replace("_", "-")):
        if candidate in all_providers:
            return candidate
    return None


def prepare_updates(all_providers: dict[str, Any], root: Path) -> list[ProviderUpdate]:
    """
    Compute additive updates for every provider already in the crate (no I/O).

    Raises:
        RuntimeError: If any existing capabilities.rs cannot be parsed.
    """
    updates: list[ProviderUpdate] = []
    errors: list[str] = []

    for provider_dir in sorted((root / "src" / "providers").iterdir()):
        path = provider_dir / "capabilities.rs"
        if not path.exists() or provider_dir.name == "openai_compatible":
            continue

        provider_id = resolve_models_dev_id(provider_dir.name, all_providers)
        if provider_id is None:
            log(f"Skipping '{provider_dir.name}': not listed on models.dev")
            continue

        try:
            current = parse_capabilities_rs(path.read_text(encoding="utf-8"))
        except ValueError as e:
            errors.append(f"{path}: {e}")
            continue

        update = merge_capabilities(
            provider_dir.name,
            path,
            current,
            all_providers[provider_id].get("models", {}),
        )
        if update:
            updates.append(update)

    if errors:
        raise RuntimeError("Could not parse capabilities files:\n" + "\n".join(errors))

    return updates


def changelog_line(updates: list[ProviderUpdate]) -> str:
    """One-line CHANGELOG summary of an update run."""
    models = sum(len(u.added_models) for u in updates)
    caps = sum(len(u.added_capabilities) for u in updates)
    return (
        f"Updated model capabilities from models.dev: {models} new models and "
        f"{caps} capability updates across {len(updates)} providers."
    )


def render_update_summary(updates: list[ProviderUpdate], limit: int = 60000) -> str:
    """Render a Markdown report of an update run, suitable as a PR body."""
    out = [
        "Automated model update from [models.dev](https://models.dev).",
        "",
        changelog_line(updates),
        "",
        "Updates are additive: no model or capability is ever removed, so this "
        "is released as a patch version.",
        "",
        "| Provider | New models | Capability updates | Not on models.dev anymore (kept) |",
        "|---|---:|---:|---:|",
    ]
    for u in updates:
        out.append(
            f"| `{u.dir_name}` | {len(u.added_models)} | "
            f"{len(u.added_capabilities)} | {len(u.not_upstream)} |"
        )
    out.append("")

    details: list[str] = []
    for u in updates:
        details.append(f"<details><summary><code>{u.dir_name}</code></summary>\n")
        if u.added_models:
            details.append("**New models**\n")
            details.extend(f"- `{m}`" for m in u.added_models)
            details.append("")
        if u.added_capabilities:
            details.append("**New capabilities**\n")
            details.extend(f"- `{m}`: {', '.join(c)}" for m, c in u.added_capabilities)
            details.append("")
        details.append("</details>\n")

    body = "\n".join(out)
    detail_text = "\n".join(details)
    if len(body) + len(detail_text) > limit:
        detail_text = "_Per-provider details omitted (too long); see the diff._\n"
    return body + "\n" + detail_text


def add_changelog_entry(changelog: Path, line: str):
    """
    Add a bullet under `### Changed` in the `[Unreleased]` section.

    Raises:
        RuntimeError: If the `[Unreleased]` header is missing.
    """
    content = changelog.read_text(encoding="utf-8")
    header = "## [Unreleased] - ReleaseDate"
    start = content.find(header)
    if start == -1:
        raise RuntimeError(f"'{header}' not found in {changelog}")

    section_start = start + len(header)
    next_release = content.find("\n## [", section_start)
    section_end = next_release if next_release != -1 else len(content)
    section = content[section_start:section_end]

    changed_idx = section.find("### Changed\n")
    if changed_idx != -1:
        insert_at = section_start + changed_idx + len("### Changed\n")
        # Keep the blank line after the heading before the new bullet
        if content[insert_at : insert_at + 1] == "\n":
            insert_at += 1
        addition = f"- {line}\n"
    else:
        insert_at = section_start
        addition = f"\n\n### Changed\n\n- {line}"

    changelog.write_text(content[:insert_at] + addition + content[insert_at:], encoding="utf-8")
    log(f"Added CHANGELOG entry to {changelog}")


# ============================================================================
# OPENAI-COMPATIBLE PROVIDER GENERATION
# ============================================================================


def generate_openai_compatible_content(
    provider_id: str, provider_data: dict[str, Any]
) -> str:
    """
    Generate mod.rs content for an OpenAI-compatible provider.

    Args:
        provider_id: Provider identifier (kebab-case)
        provider_data: Provider configuration from API

    Returns:
        Generated Rust code as string
    """
    provider_struct_name = to_pascal_case(provider_id)

    # Get API endpoint from provider_data
    api_endpoint = provider_data.get("api", "")

    # Get environment variable name from provider_data
    env_var = provider_data.get("env", [""])[0]

    # Use provider_id as default model or get from provider_data
    default_model = provider_data.get("model", provider_id)

    # Generate the Rust code with proper formatting
    lines = [
        f"//! This module provides the {
            provider_struct_name
        } provider, wrapping OpenAI Chat Completions for {
            provider_struct_name
        } requests.",
        "",
        "pub mod capabilities;",
        "",
        "// Generate the settings module",
        "crate::openai_compatible_settings!(",
        f"    {provider_struct_name}ProviderSettings,",
        f"    {provider_struct_name}ProviderSettingsBuilder,",
        f'    "{provider_struct_name}",',
        f'    "{api_endpoint}",',
        f'    "{env_var}"',
        ");",
        "",
        "// Generate the provider struct and builder",
        "crate::openai_compatible_provider!(",
        f"    {provider_struct_name},",
        f"    {provider_struct_name}Builder,",
        f"    {provider_struct_name}ProviderSettings,",
        f'    "{default_model}"',
        ");",
        "",
        "// Generate the language model implementation",
        f"crate::openai_compatible_language_model!({provider_struct_name});",
        "",
    ]

    return "\n".join(lines)


def prepare_openai_compatible_provider(
    provider_id: str,
    provider_data: dict[str, Any],
    with_capabilities: bool = False,
    root: Path | None = None,
) -> list[PendingWrite]:
    """
    Prepare OpenAI-compatible provider files (no I/O).

    Args:
        provider_id: Provider identifier (kebab-case)
        provider_data: Provider configuration from API
        with_capabilities: Whether to also prepare capabilities.rs
        root: Optional project root (defaults to auto-detected root)

    Returns:
        List of PendingWrite objects
    """
    if root is None:
        root = get_project_root()

    log(f"Preparing OpenAI-compatible provider '{provider_id}'")

    pending = []

    # Prepare mod.rs
    mod_content = generate_openai_compatible_content(provider_id, provider_data)
    pending.append(create_pending_write(provider_id, mod_content, "mod", root))

    # Optionally prepare capabilities.rs
    if with_capabilities:
        cap_write = prepare_provider_capabilities(provider_id, provider_data, root)
        if cap_write:
            pending.append(cap_write)

    return pending


def prepare_all_openai_compatible_providers(
    all_providers: dict[str, Any], with_capabilities: bool = False
) -> list[PendingWrite]:
    """
    Prepare all OpenAI-compatible providers (no I/O).

    Args:
        all_providers: All providers from models.dev API
        with_capabilities: Whether to also prepare capabilities.rs

    Returns:
        List of PendingWrite objects
    """
    root = get_project_root()
    compatible_providers = filter_openai_compatible_providers(all_providers)

    log(f"Found {len(compatible_providers)} OpenAI-compatible providers")

    pending_writes = []
    for provider_id, provider_data in compatible_providers.items():
        try:
            provider_writes = prepare_openai_compatible_provider(
                provider_id, provider_data, with_capabilities, root
            )
            pending_writes.extend(provider_writes)
        except Exception as e:
            log(f"Error preparing OpenAI-compatible provider '{provider_id}': {e}")

    return pending_writes


# ============================================================================
# CLI COMMANDS
# ============================================================================

app = typer.Typer(
    help="AI SDK Provider Code Generator - Generate Rust code for aisdk crate from models.dev",
    no_args_is_help=True,
)


def run_cargo_fmt(strict: bool = False):
    """
    Run cargo fmt to format generated code.

    Args:
        strict: Raise instead of warning when formatting fails. A failure
            usually means the generated code does not parse.
    """
    try:
        root = get_project_root()
        log("Running cargo fmt...")
        result = subprocess.run(
            ["cargo", "fmt", "--all"], cwd=root, capture_output=True, text=True
        )
        if result.returncode != 0:
            if strict:
                raise RuntimeError(f"cargo fmt failed: {result.stderr}")
            log(f"Warning: cargo fmt failed: {result.stderr}")
        else:
            log("Code formatted successfully")
    except OSError as e:
        if strict:
            raise RuntimeError(f"Could not run cargo fmt: {e}") from e
        log(f"Warning: Could not run cargo fmt: {e}")


@app.command()
def openai_compatible(
    provider_id: Annotated[
        Optional[str],
        typer.Argument(
            help="Specific models.dev provider ID to generate (e.g., 'deepseek', 'openrouter')"
        ),
    ] = None,
    with_capabilities: Annotated[
        bool,
        typer.Option("--with-capabilities", "-c", help="Also generate capabilities.rs"),
    ] = False,
):
    """
    Generate OpenAI-compatible provider code.

    Examples:
        provider-codegen.py openai-compatible
        provider-codegen.py openai-compatible deepseek
        provider-codegen.py openai-compatible --with-capabilities
        provider-codegen.py openai-compatible deepseek -c
    """
    try:
        # Fetch models.dev data
        all_providers = fetch_models_dev_json()

        # PHASE 1: Generate all content (no I/O)
        if provider_id:
            # Prepare specific provider
            if provider_id not in all_providers:
                log(f"Error: Provider '{provider_id}' not found in models.dev")
                raise typer.Exit(code=1)

            provider_data = all_providers[provider_id]

            # Check if it's OpenAI-compatible
            if provider_data.get("npm") != "@ai-sdk/openai-compatible":
                log(f"Error: Provider '{provider_id}' is not OpenAI-compatible")
                log(
                    f"Use 'capabilities {
                        provider_id
                    }' instead for non-OpenAI-compatible provider capabilities"
                )
                raise typer.Exit(code=1)

            pending_writes = prepare_openai_compatible_provider(
                provider_id, provider_data, with_capabilities
            )
        else:
            # Prepare all OpenAI-compatible providers
            pending_writes = prepare_all_openai_compatible_providers(
                all_providers, with_capabilities
            )

        # PHASE 2: Write all files atomically
        written_files = batch_write_files(pending_writes)

        # PHASE 3: Update Cargo.toml and mod.rs
        # Collect provider IDs from written files
        generated_provider_ids = []
        if provider_id:
            generated_provider_ids = [provider_id]
        else:
            generated_provider_ids = list(
                filter_openai_compatible_providers(all_providers).keys()
            )
        update_cargo_toml(generated_provider_ids)
        update_providers_mod_rs()

        # PHASE 4: Format
        run_cargo_fmt()

        # Report success
        log(f"✓ Successfully wrote {len(written_files)} files")

    except Exception as e:
        log(f"Error: {e}")
        raise typer.Exit(code=1)


@app.command()
def capabilities(
    provider_id: Annotated[
        Optional[str],
        typer.Argument(
            help="Specific provider ID to generate (e.g., 'openai', 'anthropic')"
        ),
    ] = None,
):
    """
    Generate capabilities.rs for providers.

    Examples:
        provider-codegen.py capabilities
        provider-codegen.py capabilities openai
        provider-codegen.py capabilities anthropic
    """
    try:
        # Fetch models.dev data
        all_providers = fetch_models_dev_json()

        # PHASE 1: Generate all content (no I/O)
        if provider_id:
            # Prepare specific provider capabilities
            if provider_id not in all_providers:
                log(f"Error: Provider '{provider_id}' not found in models.dev")
                raise typer.Exit(code=1)

            provider_data = all_providers[provider_id]
            cap_write = prepare_provider_capabilities(provider_id, provider_data)
            pending_writes = [cap_write] if cap_write else []

            if not pending_writes:
                log(f"Warning: No capabilities to generate for '{provider_id}'")
        else:
            # Prepare all provider capabilities
            pending_writes = prepare_all_capabilities(all_providers)

        # PHASE 2: Write all files atomically
        written_files = batch_write_files(pending_writes)

        # PHASE 3: Sync mod.rs codegen block
        update_providers_mod_rs()

        # PHASE 4: Format
        run_cargo_fmt()

        # Report success
        log(f"✓ Successfully wrote {len(written_files)} files")

    except Exception as e:
        log(f"Error: {e}")
        raise typer.Exit(code=1)


@app.command()
def update(
    summary: Annotated[
        Optional[Path],
        typer.Option(help="Write a Markdown report of the changes here (PR body)"),
    ] = None,
    changelog: Annotated[
        Optional[Path],
        typer.Option(help="Add an entry under [Unreleased] in this CHANGELOG"),
    ] = None,
    dry_run: Annotated[
        bool, typer.Option("--dry-run", help="Report changes without writing")
    ] = False,
):
    """
    Additively sync existing providers' capabilities with models.dev.

    Only providers already in the crate are touched. Models and capabilities
    are only ever added, never removed, so the result is always releasable as
    a patch version. This is what the daily `update-models` workflow runs.

    Examples:
        provider-codegen.py update
        provider-codegen.py update --dry-run
        provider-codegen.py update --summary summary.md --changelog CHANGELOG.md
    """
    try:
        root = get_project_root()
        all_providers = fetch_models_dev_json()

        # PHASE 1: Compute all updates (no I/O)
        updates = prepare_updates(all_providers, root)
        if not updates:
            log("✓ Everything is up to date")
            return

        for u in updates:
            log(
                f"{u.dir_name}: +{len(u.added_models)} models, "
                f"+{len(u.added_capabilities)} capability updates"
            )

        if dry_run:
            log(f"[DRY RUN] {changelog_line(updates)}")
            return

        # PHASE 2: Write all files atomically
        batch_write_files(
            [PendingWrite(u.path, u.content, "capabilities.rs") for u in updates]
        )

        # PHASE 3: Format (fails if the generated code does not parse)
        run_cargo_fmt(strict=True)

        # PHASE 4: Reports
        if changelog:
            add_changelog_entry(changelog, changelog_line(updates))
        if summary:
            summary.write_text(render_update_summary(updates), encoding="utf-8")
            log(f"Wrote summary to {summary}")

        log(f"✓ {changelog_line(updates)}")

    except Exception as e:
        log(f"Error: {e}")
        raise typer.Exit(code=1)


def main():
    """Main entry point."""
    app()


if __name__ == "__main__":
    main()
