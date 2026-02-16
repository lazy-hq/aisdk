#!/usr/bin/env python3
"""
/// script
/// dependencies = ["typer", "requests", "tomlkit"]
/// python-version = "3.13"

AI SDK Provider Code Generator

This single-file executable generates Rust provider and capability code from 
models.dev API data.

Usage:
    uv run --python 3.13 --with typer provider-codegen.py openai-compatible
    uv run --python 3.13 --with typer provider-codegen.py openai-compatible \
    deepseek --with-capabilities
    uv run --python 3.13 --with typer provider-codegen.py capabilities
    uv run --python 3.13 --with typer provider-codegen.py capabilities openai
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
        raise Exception(f"Failed to fetch models.dev JSON: {
                        response.status_code}")

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

    # Replace hyphens with underscores and clean up
    result = provider_id.replace("-", "_")

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
    providers_dir = root / "src" / "providers"
    provider_dir = providers_dir / provider_id

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
    new_content = content[:start_idx] + new_block + \
        content[end_idx + len(end_marker):]

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

    Args:
        model_id: Model identifier
        folder_prefix: Optional folder prefix for nested models

    Returns:
        Constructor name in snake_case
    """
    if folder_prefix:
        return to_constructor_name(folder_prefix) + "_" + to_constructor_name(model_id)
    return to_constructor_name(model_id)


def get_model_type_name(model_id: str, folder_prefix: str | None = None) -> str:
    """
    Get the type name (PascalCase) for a model with optional folder prefix.

    Args:
        model_id: Model identifier
        folder_prefix: Optional folder prefix for nested models

    Returns:
        Type name in PascalCase
    """
    if folder_prefix:
        return to_pascal_case(folder_prefix) + to_pascal_case(model_id)
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
            # Only support 1 level of nesting
            log(
                f"Warning: Model ID '{
                    model_id
                }' has more than 1 level of nesting. Using only first level."
            )
            return parts[-1], parts[-2]

    return model_id, None


def generate_capabilities_rs(provider_id: str, models: dict[str, Any]) -> str:
    """
    Generate the complete capabilities.rs content.

    Args:
        provider_id: Provider identifier (kebab-case)
        models: Dictionary of model configurations from API

    Returns:
        Generated Rust code as string
    """
    provider_struct_name = to_pascal_case(provider_id)
    provider_module = provider_id_to_snake_case(provider_id)

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

    for model_id, model_data in sorted(active_models.items()):
        # Parse model ID for folder structure
        base_name, folder_prefix = parse_model_id(model_id)

        # Get model components
        model_type_name = get_model_type_name(base_name, folder_prefix)
        model_name = model_id  # Use full model ID as the model name
        constructor_name = get_model_constructor_name(base_name, folder_prefix)
        display_name = get_model_display_name(model_id, model_data)
        capabilities = get_model_capabilities(model_data)

        lines.extend(
            [
                f"        {model_type_name} {{",
                f'            model_name: "{model_name}",',
                f"            constructor_name: {constructor_name},",
                f'            display_name: "{display_name}",',
                f"            capabilities: [{', '.join(capabilities)}]",
                "        },",
            ]
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

    log(f"Preparing capabilities for '{
        provider_id}' with {len(models)} models")
    return generate_capabilities_rs(provider_id, models)


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

    content = generate_provider_capabilities_content(
        provider_id, provider_data)
    if content is None:
        return None

    return create_pending_write(provider_id, content, "capabilities", root)


def prepare_all_capabilities(
    all_providers: dict[str, Any],
) -> list[PendingWrite]:
    """
    Prepare capabilities for all providers (no I/O).

    Args:
        all_providers: All providers from models.dev API

    Returns:
        List of PendingWrite objects
    """
    root = get_project_root()
    pending_writes = []

    for provider_id, provider_data in all_providers.items():
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
    mod_content = generate_openai_compatible_content(
        provider_id, provider_data)
    pending.append(create_pending_write(provider_id, mod_content, "mod", root))

    # Optionally prepare capabilities.rs
    if with_capabilities:
        cap_write = prepare_provider_capabilities(
            provider_id, provider_data, root)
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
            log(f"Error preparing OpenAI-compatible provider '{
                provider_id}': {e}")

    return pending_writes


# ============================================================================
# CLI COMMANDS
# ============================================================================

app = typer.Typer(
    help="AI SDK Provider Code Generator - Generate Rust code for aisdk crate from models.dev",
    no_args_is_help=True,
)


def run_cargo_fmt():
    """Run cargo fmt to format generated code."""
    try:
        root = get_project_root()
        log("Running cargo fmt...")
        result = subprocess.run(
            ["cargo", "fmt", "--all"], cwd=root, capture_output=True, text=True
        )
        if result.returncode != 0:
            log(f"Warning: cargo fmt failed: {result.stderr}")
        else:
            log("Code formatted successfully")
    except Exception as e:
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
        typer.Option("--with-capabilities", "-c",
                     help="Also generate capabilities.rs"),
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
                log(f"Error: Provider '{
                    provider_id}' is not OpenAI-compatible")
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
            cap_write = prepare_provider_capabilities(
                provider_id, provider_data)
            pending_writes = [cap_write] if cap_write else []

            if not pending_writes:
                log(f"Warning: No capabilities to generate for '{
                    provider_id}'")
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


def main():
    """Main entry point."""
    app()


if __name__ == "__main__":
    main()
