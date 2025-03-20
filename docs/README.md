# Neo Contract Rust Framework Documentation

Welcome to the Neo Contract Rust Framework documentation. This framework enables developers to write smart contracts for the Neo N3 blockchain using Rust, providing a type-safe and ergonomic development experience.

## Table of Contents

1. [Framework Overview](#framework-overview)
2. [Architecture](#architecture)
3. [Crate Structure](#crate-structure)
4. [Development Workflow](#development-workflow)
5. [API Reference](#api-reference)
6. [Examples](#examples)
7. [Best Practices](#best-practices)
8. [Troubleshooting](#troubleshooting)

## Framework Overview

The Neo Contract Rust Framework provides tools and libraries for developing Neo N3 smart contracts in Rust. The framework compiles Rust code to WebAssembly (WASM), which is then converted to Neo Virtual Machine (NeoVM) bytecode for deployment on the Neo N3 blockchain.

### Key Features

- **Type Safety**: Leverage Rust's strong type system for robust contract development.
- **Ergonomic API**: Write Neo contracts with a modern, Rust-native API.
- **Macro Support**: Simplify contract development with procedural macros (in progress).
- **Development Tools**: Tools for compiling, testing, and deploying Neo contracts.

## Architecture

The Neo Contract Rust Framework follows a multi-stage compilation process:

1. **Rust Compilation**: Rust code is compiled to WebAssembly (WASM).
2. **WASM to NeoVM Conversion**: WASM bytecode is converted to NeoVM bytecode.
3. **NEF Packaging**: NeoVM bytecode is packaged into a NEF (Neo Executable Format) file.
4. **Manifest Generation**: Contract metadata is compiled into a manifest JSON file.

![Compilation Process](compilation-process.png)

The framework uses several components to achieve this:

- **neo-contract**: Core library providing Neo N3 types and functionality.
- **neo-compiler**: Tools for converting WASM to NeoVM bytecode.
- **neo-macros**: Procedural macros for simplifying contract development.

## Crate Structure

The framework consists of the following crates:

### neo-contract

Core library providing Neo N3 functionality and types.

- `types`: Neo-specific data types (Hash160, ByteString, etc.)
- `storage`: Storage context and operations
- `events`: Event handling and emission
- `context`: Runtime context utilities

### neo-compiler

Compiler for converting WASM to NeoVM bytecode.

- `converter`: Conversion from WASM to NeoVM instructions
- `neo`: NeoVM instruction set definitions
- `wasm`: WebAssembly parsing and representation
- `manifest`: Contract manifest generation
- `nef`: NEF file generation

### neo-macros

Procedural macros for contract development.

- `contract`: Main contract macro
- `method`: Method export macro
- `event`: Event definition macro
- `storage`: Storage definitions

## Development Workflow

The typical development workflow for Neo N3 smart contracts using this framework is:

1. **Setup**: Initialize a new Rust project with dependencies on the Neo Contract Rust Framework.
2. **Development**: Write the contract code using the Neo Contract Rust Framework API.
3. **Compilation**: Compile the contract to WASM and then to NEF format.
4. **Testing**: Test the contract using the provided testing utilities.
5. **Deployment**: Deploy the contract to a Neo N3 blockchain.

### Project Setup

```bash
# Initialize a new Rust project
cargo new my-neo-contract
cd my-neo-contract

# Add dependencies to Cargo.toml
# [dependencies]
# neo-contract = { path = "../neo-contract" }
# neo-macros = { path = "../neo-macros" }

# Build the contract
cargo build --target wasm32-unknown-unknown --release
```

### Contract Structure

A basic Neo N3 contract in Rust might look like this:

```rust
use neo_contract::prelude::*;
use neo_macros::contract;

#[contract]
mod token {
    use neo_contract::prelude::*;

    #[storage]
    struct TokenStorage {
        balances: Map<Address, u64>,
        total_supply: u64,
    }

    #[event]
    struct Transfer {
        from: Address,
        to: Address,
        amount: u64,
    }

    #[method]
    fn transfer(from: Address, to: Address, amount: u64) -> bool {
        // Implementation...
    }
}
```

## API Reference

For detailed API documentation, see:

- [neo-contract API](./api/neo-contract.md)
- [neo-compiler API](./api/neo-compiler.md)
- [neo-macros API](./api/neo-macros.md)

## Examples

The framework includes several examples to help you get started:

- [Hello World](./examples/hello-world.md): A simple contract that returns a greeting message.
- [NEP-17 Token](./examples/nep17-token.md): A standard token contract implementing the NEP-17 standard.
- [DAO Contract](./examples/dao.md): A decentralized autonomous organization contract.

## Best Practices

To ensure optimal performance and security in your Neo contracts:

1. **Minimize Storage Operations**: Storage operations are expensive, so minimize them.
2. **Use Appropriate Types**: Use appropriate types for your data.
3. **Handle Errors Properly**: Always handle potential errors and edge cases.
4. **Validate Inputs**: Always validate inputs to your contract methods.
5. **Follow Standards**: Follow Neo standards (e.g., NEP-17 for tokens) for interoperability.

## Troubleshooting

Common issues and solutions:

- **Compilation Errors**: Most compilation errors are due to incorrect type usage or missing imports.
- **Runtime Errors**: Runtime errors are often caused by invalid storage operations or unhandled edge cases.
- **Gas Issues**: High gas costs are usually due to inefficient storage operations or loops.

For more detailed troubleshooting, see the [Troubleshooting Guide](./troubleshooting.md).