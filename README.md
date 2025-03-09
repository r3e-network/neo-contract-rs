# Neo Contract Framework for Rust

A framework for writing Neo N3 smart contracts in Rust using an ink!-style API. This project enables Rust developers to build smart contracts for the Neo blockchain with familiar syntax and tooling.

## Overview

The Neo Contract Framework for Rust consists of three main components:

1. **neo-contract**: The core library that provides the ink!-style API for writing smart contracts
2. **neo-contract-proc-macros**: Procedural macros for the attribute-based interface
3. **neo-compiler**: A compiler that converts WebAssembly to Neo VM bytecode

The framework follows this compilation flow:
```
Rust code → WebAssembly → Neo VM bytecode → NEF file + Manifest
```

## Features

- **Rust-First Development**: Write smart contracts in pure Rust with familiar syntax
- **ink!-Style API**: Use attribute macros like `#[contract]`, `#[storage]`, `#[method]`
- **Storage Abstractions**: Type-safe storage primitives (StorageItem, StorageMap)
- **Neo VM Compatibility**: Automatic conversion to Neo VM bytecode
- **Safe Methods**: Mark methods as safe (read-only) for improved security
- **ABI Generation**: Automatic generation of contract ABIs
- **Standard Implementation Helpers**: Utilities for implementing NEP standards (NEP-17, NEP-11, etc.)
- **Security Features**: Reentrancy protection, access control, and other security primitives
- **Comprehensive Testing**: Tools and utilities for unit testing and integration testing

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

Check out the examples directory for various contract examples:

- [Hello World](examples/hello_world/): A simple greeting contract
- [NEP-17 Token](examples/nep17/): A fungible token implementation
- [Contract Call](examples/contract_call/): Demonstrates calling other contracts
- [ink! Style Token](examples/ink_style_token/): Token implementation using ink! style
- [ink! Style Complete](examples/ink_style_complete/): A comprehensive contract example

## Documentation

For more detailed documentation:

- [Getting Started](docs/getting_started.md): Introduction to the framework
- [Storage and Variables](docs/storage_and_variables.md): Guide to storage and state management
- [NEP-17 Tutorial](docs/nep17_tutorial.md): Creating fungible tokens
- [Testing Guide](docs/testing_guide.md): Comprehensive testing strategies
- [Neo Compiler Implementation](docs/neo_compiler_implementation.md): Details on the WASM to Neo VM compiler
- [ink! Style Guide](docs/ink_style_guide.md): Patterns for ink!-style smart contracts
- [Deployment Guide](docs/deployment_guide.md): Deploying contracts to Neo networks
- [Safe Methods](docs/safe_methods.md): Using and implementing safe (read-only) methods

## Architecture

The framework architecture consists of:

1. **neo-contract**:
   - Storage abstractions for handling on-chain data
   - Runtime interaction with the Neo blockchain
   - Type conversions and serialization
   - Helper traits for standard implementations
   - Security utilities and access control

2. **neo-contract-proc-macros**:
   - Contract attribute macros for ink!-style contracts
   - Storage struct processing for state management
   - Method visibility and safety attributes
   - ABI generation for contract interfaces
   - Event definition and emission

3. **neo-compiler**:
   - WebAssembly parsing and analysis
   - WASM to Neo VM bytecode conversion
   - Intermediate representation optimization
   - NEF file generation for deployment
   - Contract manifest creation with permissions and standards

## Testing

The framework includes comprehensive testing utilities to ensure your smart contracts are reliable and secure:

- **Unit Testing**: Test individual contract methods and components
- **Integration Testing**: Test interactions between multiple contracts
- **Mock Environment**: Test with mocked blockchain state and operations
- **Storage Testing**: Verify storage operations without deploying to a blockchain
- **Security Testing**: Test reentrancy protection, access control, and more

For more details, see the [Testing Guide](docs/testing_guide.md).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

This project is inspired by [ink!](https://github.com/paritytech/ink) for Substrate and the Neo blockchain community.
