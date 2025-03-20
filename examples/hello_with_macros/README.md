# Hello World with NEO Contract Annotations

This example demonstrates how to structure a Neo N3 smart contract using the annotation-based approach, which provides documentation and structural information while implementing the contract functionality directly.

## Contract Structure

This example includes:

1. **Contract Module**: The module that contains all contract components
2. **Event Definitions**: Structures that define events to be emitted
3. **Storage Structure**: Container for contract storage fields
4. **Methods**: Contract functionality including constructor, update methods, and read-only methods

## Annotation Approach

While the current version of the framework doesn't fully implement all annotations, this example shows how the annotation syntax should be used in future versions. Currently, annotations are commented out for documentation purposes:

```rust
// #[contract]
// #[contract_author("NEO Rust Team")]
// #[contract_description("A hello world contract with attribute macros")]
// #[contract_version("0.1.0")]
mod hello_contract {
    // #[event]
    pub struct MessageUpdated {
        // #[index]
        pub from: H160,
        pub old_message: String,
        pub new_message: String,
    }
    
    // #[storage]
    pub struct HelloContract {
        // Storage fields
    }
    
    impl HelloContract {
        // #[constructor]
        pub fn new() -> Self {
            // Implementation
        }
        
        // #[method]
        pub fn update_message(&mut self, new_message: ByteString) -> bool {
            // Implementation
        }
        
        // #[safe]
        pub fn get_message(&self) -> ByteString {
            // Implementation
        }
    }
}
```

In a future version of the framework with full annotation support, you would uncomment these annotations to enable the following functionality:

1. **#[contract]**: Automatic manifest generation and contract registration
2. **#[event]**: Automatic event emission capabilities with typesafe struct-based events
3. **#[index]**: Support for filterable event indexing
4. **#[storage]**: Automatic storage initialization and serialization
5. **#[constructor]**, **#[method]**, **#[safe]**: Automatic method registration and parameter handling

## Manual Implementation

This example currently implements the contract functionality manually:

1. **Event Emission**: Uses `Runtime::notify()` with manually constructed parameters
2. **Storage**: Manually initializes storage items with keys
3. **Entry Points**: Manually implements `deploying()` and `invoke()` functions

## Building the Contract

```bash
# Build the WebAssembly binary
cargo build --release --target wasm32-unknown-unknown

# Compile to NEO VM bytecode
cd ../..
neo-compiler compile target/wasm32-unknown-unknown/release/hello_with_macros.wasm --output build/
```

## Future Improvements

In future versions of the framework, these annotations will be fully implemented, eliminating the need for manual workarounds while preserving the expressive and type-safe annotation syntax.

## Documentation

For more details on the annotation syntax, see:
- [Annotation Syntax](../../docs/ANNOTATION-SYNTAX.md)
- [Attribute Macros Guide](../../docs/ATTRIBUTE-MACROS.md) 