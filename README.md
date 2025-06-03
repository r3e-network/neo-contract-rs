# Neo Contract Rust Framework

This framework allows you to write Neo N3 smart contracts in Rust. It provides a set of libraries and utilities to make writing Neo smart contracts more ergonomic and type-safe.

## Structure

The framework is divided into several crates:

- `neo-contract`: Core library that provides the Neo N3 functionality and types
- `neo-macros`: Procedural macros for contract development (annotations)
- `neo-compiler`: Compiler that converts WASM binaries to NEO executable format
- Examples: Various examples showing how to use the framework

## Features

- **Annotation System**: Write contracts using intuitive annotations like `#[neo_contract::contract]`, `#[method]`, etc.
- **Type-Safe Storage**: Strongly typed storage primitives that provide compile-time safety
- **Event System**: Structured events with optional indexing for better off-chain integration
- **WASM Compilation**: Compile contracts from Rust to Neo VM bytecode via WebAssembly
- **Comprehensive Examples**: Various examples from simple tokens to complex DeFi applications

## Annotation System

Neo Contract Rust uses annotations to simplify contract development:

```rust
#[neo_contract::event]
pub struct Transfer {
    #[index]
    pub from: Option<H160>,
    #[index]
    pub to: Option<H160>,
    pub amount: u64,
}

#[neo_contract::contract]
pub struct TokenContract {
    #[storage]
    balances: StorageMap<H160, u64>,
    #[storage]
    total_supply: StorageItem<u64>
}

impl TokenContract {
    #[constructor]
    pub fn new(owner: H160) -> Self {
        // Initialize contract
    }
    
    #[safe]
    pub fn balance_of(&self, account: H160) -> u64 {
        // Read-only implementation
    }
    
    #[method]
    #[no_reentrant]
    pub fn transfer(&mut self, from: H160, to: H160, amount: u64) -> bool {
        // Transfer implementation with reentrancy protection
    }
}
```

For detailed documentation on the annotation system, see [examples/README.md](examples/README.md) which includes comprehensive annotation syntax and usage examples.

## Current Status

This framework is production-ready for Neo N3 smart contract development in Rust. The annotation system is fully implemented and provides a clean, declarative way to write Neo contracts.

### Macro Implementation 

The macro functionality is fully implemented in `neo-macros` and provides:

1. Contract structure with `#[neo_contract::contract]`
2. Event definition with `#[neo_contract::event]` and field indexing with `#[index]`
3. Storage field definition with `#[storage]`
4. Method annotations for constructors, state-changing methods, and read-only methods
5. Security features like reentrancy protection with `#[no_reentrant]`

### Using the Framework

To use the framework:

1. See `examples/documentation_example` for a complete example of the recommended syntax
2. Follow the patterns shown in `examples/annotation_test` for a comprehensive implementation using all annotations
3. For specific features, check the specialized examples in the `examples` directory

## Building and Testing

To build the framework:

```bash
cargo build
```

To run the examples:

```bash
# Build a specific example
cargo build -p documentation_example

# Compile to Neo VM bytecode
cargo run -p neo-compiler -- compile target/wasm32-unknown-unknown/debug/documentation_example.wasm
```

This generates:
- `.nef` file (Neo Executable Format)
- Contract manifest with all methods and events
- Debug information

## Examples

The framework includes various examples in the `examples/` directory:

- **NEP-17 Token**: Standard fungible token implementation
- **NFT Contracts**: Non-fungible token examples including basic and marketplace implementations  
- **DeFi Applications**: Decentralized finance examples
- **Cross-Contract Communication**: Inter-contract interaction patterns
- **Exchange Contracts**: Trading and swap functionality
- **DAO Examples**: Decentralized autonomous organization implementations

For detailed information about available examples and their usage, see the [Examples README](examples/README.md).

## Documentation

### Core Guides
- [Annotation Reference](docs/ANNOTATIONS.md): Complete guide to all available annotations and their usage
- [Events Guide](docs/events_guide.md): Comprehensive guide to defining, emitting, and handling events
- [Storage Guide](docs/storage_guide.md): Storage patterns, optimization techniques, and best practices
- [Contract Security Guide](docs/contract_security_guide.md): Security best practices and vulnerability prevention
- [Gas Optimization Guide](docs/gas_optimization.md): Strategies for reducing gas consumption
- [Cross-Contract Communication Guide](docs/cross_contract_guide.md): Inter-contract interaction patterns

### Framework Documentation
- [Examples README](examples/README.md): Annotation syntax examples and usage patterns
- [Macro Architecture](neo-contract/MACROS.md): Detailed documentation on macro organization and implementation
- [Contract Organization](neo-contract/CONTRACTS.md): Overview of the contract module organization
- [Compilation Guide](COMPILE_GUIDE.md): Guide to compiling contracts to Neo VM bytecode
- [WASM Toolchain](WASM_TOOLCHAIN.md): Information about the WebAssembly toolchain setup

## License

This project is licensed under [LICENSE].