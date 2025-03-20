# Neo Contract Compiler

The Neo Contract Compiler is a tool for compiling WebAssembly (WASM) smart contracts into Neo Virtual Machine (NeoVM) bytecode for deployment on the Neo N3 blockchain.

## Features

- Convert WebAssembly modules to Neo Executable Format (NEF) files
- Generate contract manifests with metadata, permissions, and ABI information
- Optimize contract bytecode for efficiency and cost
- Generate debug information for improved developer experience
- Support for various contract types and templates

## Usage

### Basic Usage

```rust
use neo_compiler::{compile, CompilerOptions};

// Compile a WASM file to NEF and manifest
let result = compile("path/to/contract.wasm")?;
println!("NEF file created at: {}", result.nef_path.display());
println!("Manifest file created at: {}", result.manifest_path.display());
```

### Advanced Options

```rust
use neo_compiler::{compile_with_options, CompilerOptions, ManifestOverride};
use std::path::PathBuf;

// Customize the compilation process
let options = CompilerOptions {
    optimize: true,
    debug: true,
    manifest_template: Some("path/to/template.json".to_string()),
    manifest_overrides: Some(vec![
        ManifestOverride {
            key: "name".to_string(),
            value: "MyCustomContract".to_string(),
        },
        ManifestOverride {
            key: "author".to_string(),
            value: "Neo Developer".to_string(),
        },
    ]),
    output_dir: Some(PathBuf::from("./output")),
    contract_name: Some("my_contract".to_string()),
};

let result = compile_with_options("path/to/contract.wasm", options)?;
```

## Compilation Process

1. The compiler loads and validates the WASM binary
2. It translates WASM instructions to NeoVM opcodes
3. Optimization passes are applied (when enabled)
4. Debug information is generated (when enabled)
5. NEF and manifest files are created in the output directory

## Manifest Generation

The compiler generates a contract manifest that includes:

- Basic metadata (name, description, etc.)
- ABI definition (methods and events)
- Required permissions
- Supported standards
- Trust settings

You can customize the manifest by providing:

1. A template manifest file as a starting point
2. Key-value overrides for specific fields

## Debug Information

When debug mode is enabled, the compiler generates a JSON file with:

- Source code mappings
- Variable information
- Function descriptions
- Sequence points for debugging

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
neo-compiler = "0.1.0"
```

## Command Line Interface

The compiler is also available as a command-line tool:

```bash
cargo install neo-compiler-cli

# Basic usage
neo-compiler ./contract.wasm

# With options
neo-compiler ./contract.wasm --optimize --debug --output ./output
```

## Requirements

- Rust 1.60 or later
- WASM files produced by supported toolchains (e.g., Rust with wasm32-unknown-unknown target)

## License

Licensed under the MIT License.