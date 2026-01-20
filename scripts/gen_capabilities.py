#!/usr/bin/env python3
"""
Generate Rust capabilities.rs files from TOML model definitions.

Usage: python gen_capabilities.py <provider_struct_name>

Example: python gen_capabilities.py OpenAI

Note: The provider name must match the Rust provider struct (e.g., 'OpenAI' not 'openai').
"""

import shutil
import sys
import subprocess
import tomllib
from pathlib import Path
from typing import Dict, List

def log(message: str):
    """Log progress messages."""
    print(f"[INFO] {message}")

def print_help(include_description: bool = False):
    """Print help message."""
    print("Usage: python gen_capabilities.py <provider> [folder_name]")
    print()
    print("Arguments:")
    print("  provider      - Provider struct name in PascalCase (e.g., 'OpenRouter', 'AmazonBedrock')")
    print("  folder_name   - (Optional) Folder name in models.dev (e.g., 'amazon-bedrock')")
    print()
    print("Examples:")
    print("  python gen_capabilities.py OpenRouter")
    print("  python gen_capabilities.py AmazonBedrock amazon-bedrock")
    print()
    print("Note: If your provider folder in models.dev doesn't match the lowercase")
    print("      version of your struct name, provide the folder name as the second argument.")
    if include_description:
        print()
        print("Description:")
        print("This script generates Rust capabilities.rs files from TOML model definitions in models.dev.")

def ensure_models_dev(root: Path):
    """Clone the models.dev repository (always fresh, shallow clone)."""
    models_dev_path = root / 'models.dev'

    if models_dev_path.exists():
        log("Removing existing models.dev...")
        shutil.rmtree(models_dev_path)

    log("Cloning models.dev repository (shallow)...")
    result = subprocess.run(
        ['git', 'clone', '--depth', '1', '--quiet', 'git@github.com:sst/models.dev.git', str(models_dev_path)],
        capture_output=True,
        text=True
    )
    if result.returncode != 0:
        log(f"Error: Failed to clone models.dev: {result.stderr}")
        sys.exit(1)

def load_provider_models(root: Path, provider_name: str) -> Dict[str, dict]:
    """Load all TOML model configs for a provider, including nested folders."""
    provider_path = root / 'models.dev' / 'providers' / provider_name / 'models'

    if not provider_path.exists():
        log(f"No models directory found for provider '{provider_name}'")
        return {}

    models = {}
    for toml_file in provider_path.rglob('*.toml'):
        # Determine if file is in a subdirectory
        relative_path = toml_file.relative_to(provider_path)
        folder_prefix = None

        # Validate nesting depth (only allow 1 level: models/*.toml or models/folder/*.toml)
        if len(relative_path.parts) > 2:
            log(f"Error: Nested folder depth exceeds 1 level at '{relative_path}'.")
            log(f"Only 'models/*.toml' and 'models/folder/*.toml' are supported.")
            raise ValueError(
                f"Nested folder depth exceeds 1 level at '{relative_path}'. "
                f"Only 'models/*.toml' and 'models/folder/*.toml' are supported."
            )

        if len(relative_path.parts) == 2:
            # File is in a subdirectory (e.g., meta-llama/llama-guard-4-12b.toml)
            folder_prefix = relative_path.parts[0]

        try:
            with open(toml_file, 'rb') as f:
                config = tomllib.load(f)

                # Skip deprecated models
                if config.get('status') == 'deprecated':
                    log(f"Skipping deprecated model: {toml_file.name}")
                    continue

                config['filename'] = toml_file.stem
                config['folder_prefix'] = folder_prefix

                # Generate model_key with folder prefix
                if folder_prefix:
                    model_key = to_pascal_case(folder_prefix) + to_pascal_case(toml_file.stem)
                else:
                    model_key = to_pascal_case(toml_file.stem)

                models[model_key] = config
                log(f"Loaded active model: {model_key} from {relative_path}")
        except Exception as e:
            log(f"Warning: Failed to load {toml_file}: {e}")

    return models

def to_pascal_case(s: str) -> str:
    """Convert string to PascalCase: gpt-3.5-turbo → Gpt35Turbo"""
    # Replace dots, hyphens, and colons with underscores, then PascalCase
    cleaned = s.replace('.', '_').replace('-', '_').replace(':', '_')
    return ''.join(word.capitalize() for word in cleaned.split('_') if word)

def to_constructor_name(s: str) -> str:
    """Convert string to lowercase constructor name: gpt-3.5-turbo → gpt_3_5_turbo"""
    # Replace dots, hyphens, colons, and parentheses with underscores, then lowercase
    cleaned = s.replace('.', '_').replace('-', '_').replace(':', '_').replace('(', '_').replace(')', '_')
    # Remove consecutive underscores and trailing underscores
    import re
    cleaned = re.sub(r'_+', '_', cleaned).strip('_')
    return cleaned.lower()

def get_capabilities(toml_data: dict) -> List[str]:
    """Extract all capabilities from TOML data based on schema."""
    capabilities = []

    # Direct boolean capability fields
    if toml_data.get('tool_call', False):
        capabilities.append('ToolCallSupport')

    if toml_data.get('reasoning', False):
        capabilities.append('ReasoningSupport')

    if toml_data.get('structured_output', False):
        capabilities.append('StructuredOutputSupport')

    # Modalities-based capabilities
    modalities = toml_data.get('modalities', {})
    input_modalities = modalities.get('input', [])
    output_modalities = modalities.get('output', [])

    # Input capabilities
    if 'text' in input_modalities:
        capabilities.append('TextInputSupport')

    if 'audio' in input_modalities:
        capabilities.append('AudioInputSupport')

    # Image input: either attachment=true OR "image" in modalities.input
    if 'image' in input_modalities or toml_data.get('attachment', False):
        capabilities.append('ImageInputSupport')

    if 'video' in input_modalities:
        capabilities.append('VideoInputSupport')

    # Output capabilities
    if 'text' in output_modalities:
        capabilities.append('TextOutputSupport')

    if 'audio' in output_modalities:
        capabilities.append('AudioOutputSupport')

    if 'image' in output_modalities:
        capabilities.append('ImageOutputSupport')

    if 'video' in output_modalities:
        capabilities.append('VideoOutputSupport')

    # Remove duplicates and sort
    return sorted(list(set(capabilities)))

def get_model_name(toml_data: dict, filename: str) -> str:
    """Get the model name constant with folder prefix if applicable."""
    # Try explicit model_name field first (if it exists in schema)
    if 'model_name' in toml_data:
        return toml_data['model_name']

    # Get base name from filename
    base_name = toml_data.get('filename', filename.replace('.toml', ''))

    # Add folder prefix if it exists
    folder_prefix = toml_data.get('folder_prefix')
    if folder_prefix:
        return f"{folder_prefix}/{base_name}"

    return base_name

def generate_capabilities_rs(provider_input: str, models: Dict[str, dict]) -> str:
    """Generate the complete capabilities.rs content."""
    provider_title = provider_input  # Keep original casing for struct name
    provider_module = provider_input.lower()  # Lowercase for module path

    lines = [
        f'//! Capabilities for {provider_module} models.',
        '//!',
        f'//! This module defines model types and their capabilities for {provider_module} providers.',
        '//! Users can implement additional traits on custom models.',
        '',
        'use crate::core::capabilities::*;',
        'use crate::model_capabilities;',
        f'use crate::providers::{provider_module}::{provider_title};',
        '',
        'model_capabilities! {',
        f'    provider: {provider_title},',
        '    models: {',
    ]

    # Generate model entries
    for _, config in sorted(models.items()):
        capabilities = get_capabilities(config)
        model_name = get_model_name(config, "")
        base_name = config['filename']
        folder_prefix = config.get('folder_prefix')

        # Create constructor name with folder prefix if applicable
        if folder_prefix:
            constructor_name = to_constructor_name(folder_prefix) + '_' + to_constructor_name(base_name)
            model_key = to_pascal_case(folder_prefix) + to_pascal_case(base_name)
        else:
            constructor_name = to_constructor_name(base_name)
            model_key = to_pascal_case(base_name)

        display_name = config['name']

        lines.extend([
            f'        {model_key} {{',
            f'            model_name: "{model_name}",',
            f'            constructor_name: {constructor_name},',
            f'            display_name: "{display_name}",',
            f'            capabilities: [{", ".join(capabilities)}]',
            '        },',
        ])

    lines.extend([
        '    }',
        '}',
    ])

    return '\n'.join(lines) + '\n'

def cleanup_models_dev(root: Path):
    """Cleanup models.dev directory."""
    models_dev_path = root / 'models.dev'
    if models_dev_path.exists():
        try:
            shutil.rmtree(models_dev_path)
            log("Cleaned up models.dev")
        except Exception as e:
            log(f"Warning: Failed to cleanup models.dev: {e}")

def main():
    """Main entry point."""
    if len(sys.argv) == 2 and sys.argv[1] == '--help':
        print_help(include_description=True)
        sys.exit(0)

    if len(sys.argv) == 2:
        provider_input = sys.argv[1]  # Keep original casing
        provider_name = provider_input.lower()  # Lowercase for file operations
        folder_name = provider_name  # Use lowercase version for folder lookup
    elif len(sys.argv) == 3:
        provider_input = sys.argv[1]  # Keep original casing for struct name
        provider_name = provider_input.lower()  # Lowercase for file operations
        folder_name = sys.argv[2]  # Use provided folder name for models.dev lookup
    else:
        print_help(include_description=False)
        sys.exit(1)

    # Determine project root
    root = Path(__file__).resolve().parent.parent
    if not (root / 'Cargo.toml').exists():
        log("Error: Could not find project root (missing Cargo.toml)")
        sys.exit(1)

    try:
        log(f"Generating capabilities for provider: {provider_name}")

        # Ensure models.dev repository is available
        ensure_models_dev(root)

        # Load model configurations - use folder_name if provided, otherwise use provider_name
        models = load_provider_models(root, folder_name)
        if not models:
            raise ValueError(f"No models found for provider '{provider_name}'")

        log(f"Found {len(models)} models")

        # Generate capabilities.rs content
        capabilities_content = generate_capabilities_rs(provider_input, models)

        # Write to output file
        output_file = root / 'src' / 'providers' / provider_name / 'capabilities.rs'
        output_file.parent.mkdir(parents=True, exist_ok=True)

        with open(output_file, 'w', encoding='utf-8') as f:
            f.write(capabilities_content)

        log(f"Generated {output_file} with {len(models)} models")

    except Exception as e:
        log(f"Error: {e}")
        cleanup_models_dev(root)
        sys.exit(1)

    # Cleanup models.dev on success
    cleanup_models_dev(root)

if __name__ == '__main__':
    main()
