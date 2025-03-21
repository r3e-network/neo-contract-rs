# neo-contract-rs

A Rust-based development framework for writing Neo N3 smart contracts. This framework allows developers to write Neo smart contracts using Rust, leveraging Rust's safety, performance, and modern development features.

## Overview

neo-contract-rs provides a comprehensive toolkit for developing Neo N3 smart contracts in Rust. The framework implements Neo's contract interface specifications and provides Rust-friendly abstractions for common Neo contract operations, including:

- Smart contract declaration and implementation
- NEP-17 token standard implementation
- Storage operations
- Runtime environment interactions
- Events and notifications
- Cryptographic operations
- Automatic manifest generation for Neo N3 contracts
- Documentation extraction for rich contract manifests

## Project Structure

- `neo-contract/` - Core library containing the Neo contract interface
- `neo-contract-proc-macros/` - Procedural macros for contract development
- `neo-wasm/` - WebAssembly to Neo executable format (NEF) converter with manifest generation
- `examples/` - Example contracts demonstrating framework usage
- `docs/` - Comprehensive documentation for the framework

## Getting Started

### Prerequisites

- Rust toolchain with wasm32-unknown-unknown target
- Cargo and make
- Go (for building the neo-wasm tool)

### Building an Example Contract

```bash
# Clone the repository
git clone https://github.com/R3E-Network/neo-contract-rs.git
cd neo-contract-rs

# Build the NEP-17 token example
cd examples/nep17
make

# Or use the compile-neo.sh script
./compile-neo.sh examples/nep17
```

## Features

- **Type Safety**: Leverage Rust's strong type system to prevent common smart contract bugs
- **NEP Standards**: Built-in support for Neo Enhancement Proposals (NEPs)
- **Procedural Macros**: Simplify contract development with custom macros
- **WASM Output**: Contracts are compiled to WebAssembly for Neo VM execution
- **Rich Manifest Generation**: Automatically creates Neo N3 contract manifests with documentation from Rust comments
- **Documentation Integration**: Extracts descriptions and annotations from Rust doc comments
- **Method Safety Detection**: Identifies read-only methods using `@safe` annotations and naming conventions
- **Neo NEF Support**: Converts WebAssembly to Neo Executable Format (NEF) for deployment

## Contract Compilation

The framework includes a compilation script (`compile-neo.sh`) that automates the process of:

1. Compiling Rust code to WebAssembly
2. Translating WebAssembly to Neo Executable Format (NEF)
3. Automatically generating a Neo N3 contract manifest with documentation
4. Organizing output files in a specified directory

```bash
./compile-neo.sh -o ./build path/to/contract
```

## Documentation

Detailed documentation can be found in the `/docs` directory, including:

- [Manifest Generation](docs/manifest-generation.md) - How manifests are generated from code
- [Documentation Best Practices](docs/documentation-best-practices.md) - How to write effective documentation
- [Understanding NEO Manifests](docs/understanding-neo-manifests.md) - Structure of NEO contract manifests
- [Code Documentation Style](docs/code-documentation-style.md) - Style guide for code documentation
- [Efficient Smart Contracts](docs/efficient-contracts.md) - Writing optimized NEO contracts

## Examples

The repository includes several examples demonstrating different aspects of the framework:

- [Hello World](examples/hello-world/) - Basic contract with greeting functionality
- [Simple Storage](examples/simple-storage/) - Key-value storage operations
- [NEP-17 Token](examples/nep17/) - Fungible token implementation
- [NEP-11 NFT](examples/nep11-nft/) - Non-fungible token implementation
- [Documented Token](examples/documented-token/) - Well-documented token with best practices
- [Transfer](examples/transfer/) - Simple value transfer functionality
- [Oracle Price Feed](examples/oracle-price-feed/) - Oracle integration example

## License

This project is licensed under the terms of the license provided in the LICENSE file.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
