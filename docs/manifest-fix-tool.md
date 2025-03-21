# Neo Contract Manifest Fix Tool

This document describes the manifest fix tool provided in neo-contract-rs, which is designed to correct manifest files by scanning Rust source code.

## Overview

The Neo N3 blockchain requires each smart contract to have an accurate manifest file that defines the contract's methods, permissions, and other metadata. Sometimes, the automatically generated manifest may not correctly reflect all methods defined in the Rust source code.

The manifest fix tool addresses this issue by:

1. Parsing the Rust source code to extract all public methods
2. Updating the manifest file with the correct method information
3. Preserving any existing offsets or metadata that were already present

## Usage

### Using the Command Line Script

The easiest way to use the manifest fix tool is through the provided `fix-manifest.sh` script:

```bash
./fix-manifest.sh [options] <path/to/project>
```

Options:
- `-m, --manifest <file>`: Path to the manifest file (default: ./build/<project-name>.manifest.json)
- `-s, --source <file>`: Path to the Rust source file (default: <project>/src/lib.rs)
- `-h, --help`: Show help message

Example:
```bash
./fix-manifest.sh neo-wasm/examples/hello-world
```

### Using the Neo-WASM CLI Directly

You can also use the neo-wasm CLI tool directly:

```bash
neo-wasm fix-manifest --manifest <path/to/manifest.json> --source <path/to/rust-source>
```

Example:
```bash
neo-wasm fix-manifest --manifest build/my-contract.manifest.json --source contracts/my-contract/src/lib.rs
```

## How It Works

The manifest fix tool:

1. **Reads the Existing Manifest**: Loads the current manifest file to preserve offsets and other metadata
2. **Parses Rust Source**: Uses regex patterns to find all public methods in impl blocks and standalone code
3. **Detects Method Safety**: Determines if methods are safe (read-only) based on annotations and naming conventions
4. **Preserves Offsets**: Keeps the function offsets from the existing manifest where available
5. **Updates the Manifest**: Writes the corrected manifest with all methods properly listed
6. **Detects Standards**: Identifies supported standards (like NEP-17) based on implemented methods

## Features

- **Comprehensive Method Detection**: Uses multiple approaches to find all public methods
- **Safety Analysis**: Correctly marks methods as safe or unsafe based on both documentation and naming
- **Standard Detection**: Automatically detects if a contract implements the NEP-17 or other standards
- **Preservation of Metadata**: Retains function offsets and other metadata from the existing manifest
- **Detailed Output**: Shows clearly which methods were found and their safety status

## When to Use

Use the manifest fix tool when:

1. You notice methods missing from your contract's manifest
2. After making changes to your contract's public interface
3. When preparing a contract for deployment to ensure all methods are properly declared
4. After fixing compilation issues that might have affected the manifest generation

## Integration with Build Process

For an automated workflow, you can add the fix-manifest step to your build process:

```bash
# Compile the contract to WASM
cargo build --target wasm32-unknown-unknown --release

# Convert WASM to NEF
neo-wasm translate --input target/wasm32-unknown-unknown/release/my-contract.wasm --output build/my-contract.nef

# Fix the manifest
./fix-manifest.sh my-contract
```

This ensures that your manifest is always accurate with the latest changes to your contract. 