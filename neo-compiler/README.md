# Neo Contract Compiler

A compiler that converts WebAssembly modules to Neo N3 smart contracts. This allows writing Neo N3 smart contracts in Rust using an ink!-style API.

## Overview

This project enables Rust developers to write smart contracts for the Neo N3 blockchain using familiar Rust syntax and patterns inspired by the ink! smart contract framework for Substrate. The compiler takes WebAssembly modules compiled from Rust code and converts them to Neo executable format (NEF) and contract manifests that can be deployed to the Neo N3 blockchain.

## Features

- **WebAssembly to Neo VM conversion**: Compile Rust to WebAssembly, then to Neo VM bytecode
- **NEF file generation**: Create valid Neo Executable Format files
- **Manifest generation**: Automatically generate contract manifests with proper ABI definitions
- **Optimizations**: Apply various optimizations to the generated Neo VM bytecode
- **Clean API**: Easy-to-use API for working with Neo VM scripts and NEF files

## Installation

```bash
cargo install --git https://github.com/neo-project/neo-contract-rs neo-compiler
```

## Usage

### Basic Compilation

The simplest way to use the compiler is through the command line:

```bash
neo-compiler compile your_contract.wasm --output-dir ./output
```

### Programmatic Usage

You can also use the compiler programmatically in your own Rust code:

```rust
use neo_compiler::{Compiler, CompilerOptions};
use std::path::Path;

fn main() {
    // Configure the compiler
    let options = CompilerOptions {
        debug: true,
        optimize: true,
        contract_name: Some("MyContract".to_string()),
        manifest_template: None,
    };
    
    let compiler = Compiler::with_options(options);
    
    // Compile the WebAssembly module
    let wasm_path = Path::new("./your_contract.wasm");
    let output_dir = Path::new("./output");
    let contract_name = "MyContract";
    
    compiler.compile(wasm_path, output_dir, contract_name).unwrap();
    
    println!("Compilation successful!");
}
```

### Manual Script Creation

You can also manually create Neo VM scripts without WebAssembly:

```rust
use neo_compiler::{script::Script, neo::OpCode, nef::NefFile};
use std::path::Path;

fn main() {
    // Create a new script
    let mut script = Script::new();
    
    // Add instructions
    script.emit_push_data(b"Hello, Neo!").unwrap();
    script.emit_opcode(OpCode::RET);
    
    // Save the script to a file
    script.write_to_file("./output/hello.neo").unwrap();
    
    // Create a NEF file from the script
    let nef = NefFile::with_script(script.to_bytes().unwrap());
    nef.save_to("./output/hello.nef").unwrap();
    
    println!("Script created successfully!");
}
```

## Documentation

For more detailed documentation:

- **[User Guide](docs/user_guide.md)**: Comprehensive guide on using the neo-compiler
- **[Migration Guide](docs/migration_guide.md)**: Guide for migrating from previous versions
- **[Roadmap](docs/roadmap.md)**: Future development plans for the project
- **[Contributing](docs/contributing.md)**: Guidelines for contributing to the project
- **[Restructuring](RESTRUCTURING.md)**: Information about the project structure
- **API Documentation**: Run `cargo doc --open` to view the API documentation

## Examples

Several examples are provided in the `examples` directory:

- **[simple_compilation.rs](examples/simple_compilation.rs)**: Demonstrates how to compile a WebAssembly module
- **[manual_script_creation.rs](examples/manual_script_creation.rs)**: Shows how to manually create Neo scripts
- **[nep17_token.rs](examples/nep17_token.rs)**: Complete NEP-17 token implementation example

To run an example:

```bash
cargo run --example nep17_token
```

## API Overview

### Core Components

- **Compiler**: Main entry point for compilation
- **WasmModule**: Representation of a WebAssembly module
- **Script**: Representation of a Neo VM script
- **NefFile**: Neo Executable Format file
- **Manifest**: Contract manifest

### Example: Working with Scripts

```rust
use neo_compiler::{script::Script, neo::OpCode};

// Create a new script
let mut script = Script::new();

// Add simple operations
script.emit_push_integer(42);
script.emit_push_integer(58);
script.emit_opcode(OpCode::ADD);
script.emit_opcode(OpCode::RET);

// Convert to bytecode
let bytes = script.to_bytes().unwrap();
```

### Example: Working with NEF Files

```rust
use neo_compiler::{nef::NefFile, script::Script};

// Create a NEF file from a script
let script = Script::new();
let mut nef = NefFile::with_script(script.to_bytes().unwrap());

// Set compiler information
let nef = nef.with_compiler("my-compiler");

// Finalize the NEF file (calculate checksum)
nef.finalize().unwrap();

// Save to file
nef.save_to("./output/contract.nef").unwrap();
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.