# Neo N3 Contract Compiler

A compiler that converts WebAssembly modules to Neo N3 smart contracts. This tool enables writing Neo N3 smart contracts in Rust and compiling them to Neo VM bytecode for deployment on the Neo N3 blockchain.

## Overview

The `neo-compiler` is a critical component of the Neo N3 smart contract development ecosystem in Rust. It takes WebAssembly modules compiled from Rust code and converts them to the Neo Executable Format (NEF) and contract manifests required for deployment to the Neo N3 blockchain.

## Features

- **WebAssembly to Neo N3 VM Conversion**: Compile Rust to WebAssembly, then to Neo N3 VM bytecode
- **NEF File Generation**: Create valid Neo N3 Executable Format files for deployment
- **Manifest Generation**: Automatically generate contract manifests with proper ABI definitions for Neo N3
- **Safe Method Identification**: Properly mark read-only methods as "safe" in the manifest
- **Neo N3 Optimizations**: Apply optimizations specific to Neo N3 VM bytecode
- **Clean API**: Easy-to-use API for working with Neo N3 VM scripts and NEF files

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

This will produce both the NEF file and manifest required for Neo N3 deployment.

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

For advanced use cases, you can manually create Neo N3 VM scripts:

```rust
use neo_compiler::{script::Script, neo::OpCode, nef::NefFile};
use std::path::Path;

fn main() {
    // Create a new script
    let mut script = Script::new();
    
    // Add instructions
    script.emit_push_data(b"Hello, Neo N3!").unwrap();
    script.emit_opcode(OpCode::RET);
    
    // Save the script to a file
    script.write_to_file("./output/hello.neo").unwrap();
    
    // Create a NEF file from the script
    let nef = NefFile::with_script(script.to_bytes().unwrap());
    nef.save_to("./output/hello.nef").unwrap();
    
    println!("Script created successfully!");
}
```

## Neo N3 Compilation Process

The compilation process involves several stages:

1. **Parse WebAssembly**: Read and analyze the WebAssembly binary format
2. **Analyze Code**: Determine function signatures, types, and other metadata
3. **Generate Neo N3 Bytecode**: Convert WebAssembly instructions to Neo N3 VM instructions
4. **Create NEF**: Package the bytecode into the Neo N3 Executable Format
5. **Generate Manifest**: Create a contract manifest with proper metadata, permissions, and ABI

## Documentation

For more detailed documentation:

- **[User Guide](docs/user_guide.md)**: Comprehensive guide on using the neo-compiler
- **[Neo N3 Implementation Guide](../docs/neo_n3_implementation_guide.md)**: Guide for Neo N3 contract patterns and best practices
- **[Neo N3 Opcode Mapping](docs/neo_n3_opcode_mapping.md)**: Mapping between WebAssembly and Neo N3 opcodes
- **[Contract Structure](docs/contract_structure.md)**: Understanding Neo N3 contract structure
- **API Documentation**: Run `cargo doc --open` to view the API documentation

## Examples

See the [examples directory](../examples/) for complete examples of Neo N3 contracts written in Rust:

- **[NEP-17 Token](../examples/nep17/)**: Implementation of the Neo N3 fungible token standard
- **[Event Demo](../examples/event_demo/)**: Demonstration of proper Neo N3 event emission

## Contributing

Contributions to improve the Neo N3 Contract Compiler are welcome! Please see our [Contributing Guide](../CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License.