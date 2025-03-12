# Neo Contract Framework for Rust

A framework for writing Neo N3 smart contracts in Rust using an ink!-style API. This project enables Rust developers to build smart contracts for the Neo blockchain with familiar syntax and tooling.

## Overview

The Neo Contract Framework for Rust consists of the following main components:

1. **neo-contract**: The core library that provides the ink!-style API for writing smart contracts
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

We've updated and expanded our examples directory with comprehensive READMEs and improved code structure. Check out the [examples directory](examples/) for various contract implementations, including:

### Basic Examples
- [Hello World](examples/hello_world/): A simple greeting contract
- [Simple Token](examples/simple_token/): A basic token implementation
- [Event Demo](examples/event_demo/): Demonstrates event emission
- [Compilation Example](examples/compilation_example/): Shows compilation workflow with a counter contract

### Token Standards
- [NEP-17 Token](examples/nep17/): A fungible token implementation
- [NFT](examples/nft/): Non-fungible token implementation

### Governance and DeFi
- [DAO](examples/dao/): A decentralized autonomous organization
- [DeFi](examples/defi/): Decentralized finance examples

### Advanced Patterns
- [Contract Call](examples/contract_call/): Demonstrates calling other contracts
- [ink! Style Contracts](examples/ink_style_complete/): Comprehensive ink! style implementations

Each example includes detailed READMEs explaining the contract's structure, functionality, and usage patterns.

## Known Issues and Workarounds

Common issues you may encounter when working with examples:

1. **Procedural Macro Issues**: The `#[contract]`, `#[method]`, `#[safe]`, and `#[constructor]` macros may not resolve correctly
   - **Workaround**: Use the `std` feature during development
   
2. **Storage Trait Issues**: The `Storage` trait and `StorageContext` may not be found
   - **Workaround**: Import explicitly: `use neo_contract::types::storage::Storage`
   
3. **Runtime Function Signature Mismatches**: Function signatures may change between versions
   - **Workaround**: Check the current implementation in the framework source

For detailed solutions to common errors, see our [Troubleshooting Guide](docs/troubleshooting.md).

## Documentation

The framework includes comprehensive documentation to help you get started and understand advanced concepts:

- [Documentation Index](docs/index.md) - Central hub for all documentation
- [Getting Started](docs/quickstart.md) - Quick start guide
- [Storage and Variables](docs/storage_guide.md) - Working with on-chain storage
- [Ledger API Guide](docs/ledger_api_guide.md) - Accessing blockchain data
- [Contract Security Guide](docs/contract_security_guide.md) - Security best practices
- [Contract Testing Guide](docs/contract_testing_guide.md) - Testing your contracts
- [Ledger API Testing](docs/ledger_api_testing.md) - Testing blockchain data dependent contracts
- [Advanced Storage Patterns](docs/advanced_storage.md) - Efficient storage solutions
- [Gas Optimization](docs/gas_optimization.md) - Optimizing gas usage
- [Events Guide](docs/events_guide.md) - Working with events and notifications
- [Transaction Patterns](docs/transaction_patterns.md) - Transaction validation and processing
- [Cross-Contract Communication](docs/cross_contract_guide.md) - Contract-to-contract interaction
- [NEP-17 Tutorial](docs/nep17_guide.md) - Creating fungible tokens
- [Neo Contract Annotations](docs/neo_contract_annotations.md) - Using Neo's modern annotation system
- [Ledger API Workshop](docs/ledger_api_workshop.md) - Hands-on tutorial for blockchain-dependent contracts
- [Neo Compiler Implementation](docs/compiler.md) - How the compiler works
- [ink! Style Guide](docs/ink_style.md) - Coding style recommendations
- [Deployment Guide](docs/deployment.md) - Deploying your contracts
- [Safe Methods](docs/safe_methods.md) - Read-only contract methods
- [Troubleshooting](docs/troubleshooting.md) - Common issues and solutions
- [Documentation Updates](docs/DOCUMENTATION_UPDATES.md) - Summary of recent documentation enhancements

All documentation is organized in the [docs directory](docs/) with a comprehensive structure to help you find the information you need. The documentation covers everything from basic concepts to advanced topics like security, testing, and optimization.

## Architecture

The framework architecture consists of:

1. **neo-contract**:
   - Storage abstractions for handling on-chain data
   - Runtime interaction with the Neo blockchain
   - Type conversions and serialization
   - Helper traits for standard implementations
   - Security utilities and access control

2. **neo-compiler**:
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
