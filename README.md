# Neo Contract Framework for Rust

A framework for writing Neo N3 smart contracts in Rust. This project enables Rust developers to build smart contracts for the Neo N3 blockchain with familiar syntax and tooling.

## Overview

The Neo Contract Framework for Rust consists of the following main components:

1. **neo-contract**: The core library that provides the API for writing Neo N3 smart contracts
2. **neo-macros**: Procedural macros for the attribute-based interface
3. **neo-compiler**: A compiler that converts WebAssembly to Neo VM bytecode
4. **neo-contract-testing**: Testing utilities for Neo smart contracts

The framework follows this compilation flow:
```
Rust code → WebAssembly → Neo VM bytecode → NEF file + Manifest
```

## Current Status

**Important Note**: This framework is currently in active development. Some examples may experience compilation issues due to ongoing development of the procedural macros and framework components. The code is provided as a reference for contract structure and patterns, but may require updates to compile successfully.

## Features

- **Rust-First Development**: Write smart contracts in pure Rust with familiar syntax
- **Neo N3 Compatibility**: Full support for Neo N3 blockchain features and standards
- **Storage Abstractions**: Type-safe storage primitives (StorageItem, StorageMap)
- **Neo VM Compatibility**: Automatic conversion to Neo VM bytecode
- **Safe Methods**: Mark methods as safe (read-only) for improved security and gas efficiency
- **ABI Generation**: Automatic generation of contract ABIs
- **Standard Implementation Helpers**: Utilities for implementing NEP standards (NEP-17, NEP-11, etc.)
- **Security Features**: Reentrancy protection, access control, and other security primitives
- **Comprehensive Testing**: Tools and utilities for unit testing and integration testing
- **Standardized Event Handling**: Automatic generation of Neo N3-compliant event emission code

## Neo N3 Event Handling

Neo N3 smart contracts emit events to notify external applications about important state changes. The framework provides a standardized way to define and emit events following Neo N3 best practices.

### Defining Events

Events are defined as structs with the `#[event]` attribute:

```rust
#[event]
struct Transfer {
    #[index]
    from: Address,
    #[index]
    to: Address,
    amount: u64,
}
```

The `#[index]` attribute marks fields that should be indexed for efficient filtering when querying events from the blockchain.

### Emitting Events

The `#[event]` attribute automatically generates an `emit` method that properly formats and emits the event following Neo N3 standards:

```rust
// Emit the event with proper Neo N3 formatting
Transfer::emit(sender, recipient, value);
```

Under the hood, this generates code that:

1. Creates a `ByteString` with the event name
2. Creates an `Array<Any>` to hold event parameters
3. Converts parameters to `Any` type with proper null handling for Option types
4. Calls `Runtime::notify(event_name, event_params)` to emit the event

This ensures all events are emitted in a standardized way that follows Neo N3 specifications.

## Quick Start

### Installation

```bash
# Install cargo dependencies
cargo install cargo-make
cargo install wasm-strip

# Clone the repository
git clone https://github.com/neo-project/neo-contract-rs
cd neo-contract-rs

# Build all components
cargo build --release
```

### Writing a Smart Contract

Create a new crate for your contract:

```bash
cargo new --lib my-contract
cd my-contract
```

Add dependencies to your Cargo.toml:

```toml
[package]
name = "my-contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
neo-contract = { path = "../path/to/neo-contract" }

[profile.release]
lto = true
opt-level = "z"
overflow-checks = true
panic = "abort"
codegen-units = 1
```

Implement your contract:

```rust
use neo_contract::prelude::*;

#[contract]
pub struct MyContract {
    counter: StorageItem<u64>,
}

#[contractimpl]
impl MyContract {
    #[constructor]
    pub fn new() -> Self {
        Self {
            counter: StorageItem::new(0),
        }
    }
    
    #[method]
    pub fn increment(&mut self) {
        let current = self.counter.get();
        self.counter.set(current + 1);
    }
    
    #[method]
    #[safe]
    pub fn get_counter(&self) -> u64 {
        self.counter.get()
    }
}
```

### Compiling and Deploying

1. Compile to WebAssembly:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. Convert to Neo N3 smart contract:
   ```bash
   neo-compiler compile \
       target/wasm32-unknown-unknown/release/my_contract.wasm \
       --output ./build
   ```

3. Deploy using Neo CLI or other tools:
   ```
   neo-cli deploy ./build/my_contract.nef ./build/my_contract.manifest.json
   ```

## Examples

Check out the [examples directory](examples/) for various contract implementations, including:

### Basic Examples
- [Hello World](examples/hello_world/): A simple greeting contract
- [Event Demo](examples/event_demo/): Demonstrates proper Neo N3 event emission

### Token Standards
- [NEP-17 Token](examples/nep17/): A fungible token implementation following Neo N3 standards

## Safe Methods

Neo N3 contracts should distinguish between safe (read-only) and non-safe (state-modifying) methods. Safe methods are marked with the `#[safe]` attribute and are represented in the contract manifest with `"safe": true`. This is important for optimizing contract execution and ensuring proper access control.

## Documentation

The framework includes comprehensive documentation:

- [Documentation Index](docs/index.md) - Central hub for all documentation
- [Getting Started](docs/getting_started.md) - Quick start guide 
- [Storage Guide](docs/storage_guide.md) - Working with on-chain storage
- [Events Guide](docs/events_guide.md) - Working with Neo N3 events and notifications
- [Contract Security Guide](docs/contract_security_guide.md) - Security best practices
- [Neo Compiler Implementation](docs/neo_compiler_implementation.md) - How the compiler works
- [Deployment Guide](docs/deployment_guide.md) - Deploying your contracts
- [Safe Methods](docs/safe_methods.md) - Read-only contract methods
- [Troubleshooting](docs/troubleshooting.md) - Common issues and solutions

## Architecture

The Neo Contract Framework for Rust is designed with these key components:

1. **neo-contract**: Core library for contract development
   - Storage abstractions
   - Event handling
   - Contract annotations
   - Neo N3 runtime access
   
2. **neo-compiler**: Compiles WebAssembly to Neo VM bytecode
   - WASM parsing
   - Neo VM code generation
   - Contract manifest generation
   - NEF file generation

3. **neo-macros**: Procedural macros for simplified contract development
   - Contract attributes
   - Storage attributes
   - Method attributes
   - Event attributes

4. **neo-contract-testing**: Testing utilities
   - Mock runtime environment
   - Test helpers for contract execution
   - Assertion utilities

## Contributing

We welcome contributions to improve the Neo Contract Framework for Rust. See our [Contributing Guide](CONTRIBUTING.md) for details on how to contribute.

## License

This project is licensed under the [MIT License](LICENSE).
